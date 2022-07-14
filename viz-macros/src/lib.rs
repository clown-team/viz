//! Macros for Viz Web Framework
//!
//! Generators for handler
//!
//! ## Handler
//!
//! TODO: `#[handler.before(a).after(b).around(c)]
//!
//! ## Example
//!
//! ```rust
//! use viz::{IntoResponse, Result};
//! use viz_macros::handler;
//!
//! #[handler]
//! async fn index() -> impl IntoResponse {
//!     ()
//! }
//!
//! #[handler]
//! async fn get_user() -> Result<impl IntoResponse> {
//!     Ok(())
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Result, ReturnType};

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    generate_handler(args, input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn generate_handler(_args: AttributeArgs, input: TokenStream) -> Result<TokenStream> {
    let ast = syn::parse::<ItemFn>(input)?;
    let vis = &ast.vis;
    let docs = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .cloned()
        .collect::<Vec<_>>();
    let name = ast.sig.ident.clone();
    let asyncness = if ast.sig.asyncness.is_some() {
        Some(quote!(.await))
    } else {
        None
    };
    let mut out = quote!(Ok(res));
    let mut is_ok_type = false;
    match &ast.sig.output {
        // ()
        ReturnType::Default => {
            is_ok_type = true;
        }
        ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(path) => {
                if let Some(seg) = &path.path.segments.first() {
                    is_ok_type = true;
                    // T
                    // impl IntoResponse
                    // Result<T>
                    // Result<impl IntoResponse>
                    if seg.ident == "Result" {
                        out = quote!(res);
                    }
                }
            }
            syn::Type::ImplTrait(i) => {
                if let Some(syn::TypeParamBound::Trait(d)) = &i.bounds.first() {
                    // T
                    // impl IntoResponse
                    if matches!(d.path.get_ident(), Some(ident) if ident == "IntoResponse") {
                        is_ok_type = true;
                    }
                }
            }
            syn::Type::Tuple(_) => {
                // (T,...)
                is_ok_type = true;
            }
            _ => {
                is_ok_type = false;
            }
        },
    }

    if !is_ok_type {
        out = quote!();
    }

    let extractors =
        ast.sig
            .inputs
            .clone()
            .into_iter()
            .fold(Vec::new(), |mut extractors, input| {
                if let FnArg::Typed(pat) = input {
                    let ty = &pat.ty;
                    extractors.push(quote!(<#ty as viz::FromRequest>::extract(&mut req).await?));
                }
                extractors
            });

    let stream = quote! {
        #(#docs)*
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        #vis struct #name;

        #[viz::async_trait]
        impl viz::Handler<viz::Request<viz::Body>> for #name
        {
            type Output = viz::Result<viz::Response<viz::Body>>;

            #[allow(unused, unused_mut)]
            async fn call(&self, mut req: viz::Request<viz::Body>) -> Self::Output {
                #ast
                let res = #name(#(#extractors),*)#asyncness;
                #out.map(viz::IntoResponse::into_response)
            }
        }
    };

    Ok(stream.into())
}
