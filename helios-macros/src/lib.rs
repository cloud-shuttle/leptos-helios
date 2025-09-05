//! Helios Procedural Macros
//!
//! This crate provides procedural macros for Helios chart specifications.

use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

/// Chart specification macro
#[proc_macro]
pub fn chart(input: TokenStream) -> TokenStream {
    let _input = input;

    // Placeholder implementation
    quote! {
        helios_core::chart::ChartSpec::default()
    }
    .into()
}

/// Dashboard specification macro
#[proc_macro]
pub fn dashboard(input: TokenStream) -> TokenStream {
    let _input = input;

    // Placeholder implementation
    quote! {
        helios_core::chart::ChartSpec::default()
    }
    .into()
}
