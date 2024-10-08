use darling::ast::{Data, Fields, Style};
use darling::{util, FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput};


#[allow(dead_code)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
pub(crate) struct ErrorData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
    app_type: syn::Type,
    prefix: String,
}

#[allow(dead_code)]
#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<util::Ignored>,
    code: String,
    #[darling(default)]
    app_code: String,
    #[darling(default)]
    client_msg: String,
}

pub fn process_derive_to_error_info(_input: DeriveInput) -> TokenStream {
    let ErrorData {
        ident: name,
        generics,
        data: Data::Enum(data),
        app_type,
        prefix,
    } = ErrorData::from_derive_input(&_input).expect(" Can not parse input")else {
        panic!("Only enum is supported");
    };

    // #name::#ident(_) => { // code to new ErrorInfo }

    let code = data.iter()
        .map(|v| {
            let EnumVariants {
                ident,
                fields,
                code,
                app_code,
                client_msg
            } = v;
            let code = format!("{}{}", prefix, code);
            let varint_code = match fields.style{
                Style::Struct=>quote!{#name::#ident {..}},
                Style::Tuple=>quote!{#name::#ident(_)},
                Style::Unit=>quote!{#name::#ident}
            };
            // let varint_code = match fields.style{
            //
            // }
            quote! {
                #varint_code=>{
                    ErrorInfo::new(
                          #app_code,
                            #code,
                            #client_msg,
                            self
                        )
                }
             }
        }).collect::<Vec<_>>();
    quote! {
        use error_code::{ErrorInfo, ToErrorInfo as _};
        impl #generics ToErrorInfo for #name #generics {
            type T = #app_type;

            fn to_error_info(&self)->ErrorInfo<Self::T>{
                match self{
                    #(#code)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromDeriveInput;

    #[test]
    fn test_darling_data_struct() {
        let input = r#"
        #[derive(thiserror::Error, ToErrorInfo)]
        #[error_info(app_type="http::StatusCode", prefix="01")]
        pub enum MyError {
        #[error("Invalid command: {0}")]
        #[error_info(code="IC", app_code="400")]
        InvalidCommand(String),

        #[error("Invalid argument: {0}")]
        #[error_info(code="IA", app_code="400", client_msg="friendly msg")]
        InvalidArgument(String),

        #[error("{0}")]
        #[error_info(code="RE", app_code="500")]
        RespError(#[from] RespError),
        }
        "#;
        let parsed: DeriveInput = syn::parse_str(input).unwrap();
        let info = ErrorData::from_derive_input(&parsed).unwrap();
        println!(" {:#?}", info);
        assert_eq!(info.ident.to_string(),"MyError");
        // assert_eq!(info.app_type.to_string(),"http::StatusCode");
        let code = process_derive_to_error_info(parsed);
        println!("{:#?}",code);

    }
}