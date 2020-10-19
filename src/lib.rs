use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{parse_macro_input, DeriveInput};

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[proc_macro_derive(IntoDefault, attributes(IntoStruct, IntoType))]
pub fn into_default(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let attributes = &ast.attrs;
    let base_type = &ast.ident;
    let mut code: TokenStream = TokenStream::from(quote! {});
    for attribute in attributes {
        if attribute.path.is_ident("IntoStruct") {
            let type_struct = attribute.parse_args::<Ident>().unwrap();
            if let syn::Data::Struct(d) = &ast.data {
                if let syn::Fields::Named(f) = &d.fields {
                    let fields = &f.named;
                    let mut fields_name = Vec::new();
                    let mut fields_value = Vec::new();
                    for field in fields {
                        let type_temp = &field.ident;
                        let mut quote_temp = quote! {item.#type_temp.into()};
                        fields_name.push(&field.ident);
                        if let syn::Type::Path(path) = &field.ty {
                            for segment in &path.path.segments {
                                if segment.ident == "Option" {
                                    quote_temp = quote! {match item.#type_temp {
                                        Some(value) => value.into(),
                                        None => Default::default()
                                    }};
                                }
                            }
                        }
                        fields_value.push(quote_temp);
                    }
                    code = TokenStream::from(quote! {
                        impl From<Option<#base_type>> for #type_struct {
                            fn from(item: Option<#base_type>) -> Self {
                               match item {
                                   Some(v) => v.into(),
                                   None => Default::default()
                               }
                            }
                        }

                        impl Into<Option<#type_struct>> for #base_type {
                            fn into(self) -> Option<#type_struct> {
                                Some(self.into())
                            }
                        }

                        impl From<#base_type> for #type_struct {
                          fn from(item: #base_type) -> Self {
                             #type_struct {
                                #(#fields_name: #fields_value ,)*
                             }
                          }
                        }

                    });
                }
            }
        } else {
            TokenStream::from(quote! {});
        }
    }
    code
}
