use proc_macro::TokenStream;
use quote::quote;
#[proc_macro_derive(IntoDefault)]
pub fn into_default(item: TokenStream) -> TokenStream {
   let code = quote! {};
   TokenStream::from(code)
}