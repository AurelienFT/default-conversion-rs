use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{parse_macro_input, DeriveInput};

fn _print_type_of<T>(_: &T) {
   println!("{}", std::any::type_name::<T>())
}

#[proc_macro_derive(IntoDefault, attributes(IntoStruct))]
pub fn into_default(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let attributes = &ast.attrs;
    let base_type = &ast.ident;
    let mut code: TokenStream = TokenStream::from(quote! {});
    //println!("{:#?}", &ast);
    for attribute in attributes {
        if attribute.path.is_ident("IntoStruct") {
            let type_struct = attribute.parse_args::<Ident>().unwrap();
            if let syn::Data::Struct(d) = &ast.data {
               if let syn::Fields::Named(f) = &d.fields {
                  let fields = &f.named;
                  let mut fields_name = Vec::new();
                  //println!("{:#?}", &fields);
                  for field in fields {
                     fields_name.push(&field.ident)
                  }
                  //print_type_of(&fields);
                  code = TokenStream::from(quote! {
                     impl From<#base_type> for #type_struct {
                        fn from(item: #base_type) -> Self {
                           #type_struct {
                              #(#fields_name: item.#fields_name,)*
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
    TokenStream::from(code)
}
