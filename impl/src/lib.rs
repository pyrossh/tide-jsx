extern crate proc_macro;

mod child;
mod children;
mod element;
mod element_attribute;
mod element_attributes;
mod function_component;
mod tags;

use element::Element;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    let el = proc_macro2::TokenStream::from(rsx(input));
    let result = quote! { ::tide_jsx::Render::render(#el) };
    TokenStream::from(result)
}

/// Generate a renderable component tree, before rendering it
#[proc_macro]
#[proc_macro_error]
pub fn rsx(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as Element);
    let result = quote! { #el };
    TokenStream::from(result)
}

#[proc_macro]
#[proc_macro_error]
pub fn view(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as Element);
    let result = quote! {
      ::tide::Response::builder(StatusCode::Ok)
            .content_type(::tide::http::mime::HTML)
            .body(#el.render())
            .build()
    };
    TokenStream::from(result)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    function_component::create_function_component(f)
}
