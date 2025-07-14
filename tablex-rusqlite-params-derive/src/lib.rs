use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Params)]
pub fn derive_params(input: TokenStream) -> TokenStream { 
    let item_struct @ ItemStruct { .. } = parse_macro_input!(input);

    let struct_ident = &item_struct.ident;

    let params = item_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            let key = format!(":{}", field_name.as_ref().unwrap());

            quote! {
                (#key, &self.#field_name as &dyn ::rusqlite::ToSql)
            }
        });


    
    quote! {
        impl ::tablex_rusqlite::Params for #struct_ident {
            type BindIndex = &'static str;

            fn params(&self) -> impl ::std::iter::Iterator<Item = (Self::BindIndex, &dyn ::rusqlite::ToSql)> {
                [
                    #(#params),*
                ]
                .into_iter()
            }
        }
    }.into()
}