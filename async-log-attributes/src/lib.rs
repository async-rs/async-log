//! Proc Macro attributes for the `async-log` crate.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Defines the `instrument` function.
#[proc_macro_attribute]
pub fn instrument(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let asyncness = &input.asyncness;
    let abi = &input.abi;

    let generics = &input.decl.generics;
    let name = &input.ident;
    let inputs = &input.decl.inputs;
    let output = &input.decl.output;
    let body = &input.block.stmts;

    let mut names = String::new();
    let mut args = Vec::<syn::Pat>::new();

    for fn_arg in inputs {
        if let syn::FnArg::Captured(arg) = fn_arg {
            let pat = arg.pat.clone();

            if let syn::Pat::Ident(pat_ident) = &pat {
                names.push_str(&format!(", {}={{:?}}", pat_ident.ident));
            } else {
                let tokens = quote_spanned! { fn_arg.span() =>
                    compile_error!("instrumented functions need to name arguments");
                };
                return TokenStream::from(tokens);
            }

            args.push(pat);
        }
    }

    let result = quote! {
        #(#attrs)*
        #vis #constness #unsafety #asyncness #abi fn #name #generics (#(#inputs)*) #output {
            let __name = format!("{}#{}", file!(), stringify!(#name));
            let __args = format!("{}{}", __name, format_args!(#names, #(#args)*));
            async_log::span!(__args, {
                #(#body)*
            })
        }
    };

    result.into()
}
