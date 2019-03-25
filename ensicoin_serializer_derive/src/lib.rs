extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Serialize)]
pub fn serialize_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_serialize_macro(&ast)
}

fn impl_serialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let mut body = quote!{};
    match &ast.data {
        syn::Data::Struct(data) => {
            for field in data.fields.iter() {
                match &field.ident {
                    Some(field_name) => {
                        body = quote! {
                            #body
                            v.append(&mut self.#field_name.serialize());
                        }
                    }
                    None => panic!("Can't derive unamed field in {}", name),
                }
            }
        }
        _ => panic!("Can only derive struts, {} is invalid", name),
    }

    let gen = quote!{
        impl #generics Serialize for #name #generics {
            fn serialize(&self) -> Vec<u8> {
                let mut v = Vec::new();
                #body
                v
            }
       }
    };
    gen.into()
}
