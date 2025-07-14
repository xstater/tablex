use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(FromRow)]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let item_struct @ ItemStruct { .. } = parse_macro_input!(input);

    let struct_ident = &item_struct.ident;

    let getters = item_struct.fields.iter().enumerate().map(|(index, field)| {
        let field_name = field.ident.as_ref().unwrap();

        quote! {
            #field_name: row.get(#index)?
        }
    });

    quote! {
        impl ::tablex_rusqlite::FromRow for #struct_ident {
            fn from_row(row: &::rusqlite::Row) -> ::rusqlite::Result<Self>
            where Self: Sized
            {
                Ok(Self {
                    #(#getters),*
                })
            }
        }
    }
    .into()
}
