use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(IterAll)]
pub fn iter_all_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let enum_name = &input.ident;

    // Generate code for iterating over each variant of the enum
    let variant_iter_code = match input.data {
        Data::Enum(ref data_enum) => {
            let iter_code = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    action(#enum_name::#variant_name(Default::default()))
                }
            });
            quote! {
                #(#iter_code;)*
            }
        }
        _ => panic!("IterAll can only be derived for enums"),
    };

    // Generate the implementation code for IterAll trait
    quote! {
        impl iter_all::IterAll for #enum_name {
            fn iter_all(mut action: impl FnMut(Self)) {
                #variant_iter_code
            }
        }
    }
    .into()
}
