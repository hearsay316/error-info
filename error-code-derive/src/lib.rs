mod error_info;
use proc_macro::TokenStream;

#[proc_macro_derive(ToErrorInfo,attributes(error_info))]
pub fn derive_to_error_info(input:TokenStream)->TokenStream{
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    error_info::process_derive_to_error_info(input).into()
}

