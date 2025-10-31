use proc_macro2::TokenStream;
use quote::quote;

pub fn derive_parameters(item: syn::ItemStruct) -> TokenStream {
    let item_ident = item.ident;

    let mut field_functions: Vec<TokenStream> = vec![];
    
    for field in item.fields {
        let field_type = field.ty;
        let field_ident = field.ident.unwrap();

        field_functions.push(quote! {
            pub fn #field_ident(mut self, val: impl Into<#field_type>) -> Self {
                self.#field_ident = val.into();
                self
            }
        });
    }
    quote! { 
        impl #item_ident {
            #(#field_functions)*
        }
    }
}
