extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use darling::{FromDeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(self_name))]
struct SelfNameOption {
    #[darling(default)]
    lowercase: bool,
}

#[proc_macro_derive(SelfName, attributes(self_name))]
pub fn derive_self_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let option = SelfNameOption::from_derive_input(&item).unwrap();
    let struct_name = item.ident;
    let mut name_str = struct_name.to_string();

    if option.lowercase {
        name_str = name_str.to_lowercase();
    }

    let gen = quote! {
        impl #struct_name {
            pub fn self_name(&self) -> &str {
                #name_str
            }
        }
    };

    gen.into()
}
