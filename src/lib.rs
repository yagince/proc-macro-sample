extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(SelfName)]
pub fn derive_self_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.ident;
    let gen = quote! {
        impl #struct_name {
            pub fn self_name(&self) -> String {
                stringify!(#struct_name).to_owned()
            }
        }
    };

    gen.into()
}
