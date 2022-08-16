use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn pipe_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ident: syn::Ident = syn::parse(attr).unwrap();
    let item: TokenStream2 = item.into();
    quote::quote! {
        #ident ! {
            #item
        }
    }.into()
}
