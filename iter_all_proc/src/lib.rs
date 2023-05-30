use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(IterAll)]
pub fn iter_all_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let Data::Enum(ref data_enum) = input.data else  {
        panic!("IterAll can only be derived for enums");
    };

    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        quote! {
            action(#enum_name::#variant_name(Default::default()));
        }
    });

    let variants = quote! {
        #(#variants)*
    };

    let variants2 = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let Fields::Unnamed(ref unnamed) = variant.fields else {
            panic!();
        };

        let ty = &unnamed.unnamed.first().unwrap().ty;

        let lowercase_ident = format_ident!("{}", variant_name.to_string().to_lowercase());

        quote! {
            fn #lowercase_ident() -> #ty {
                Default::default()
            }
        }
    });

    let variants2 = quote! {
        #(#variants2)*
    };

    quote! {
        impl iter_all::IterAll for #enum_name {
            fn iter_all(mut action: impl FnMut(Self)) {
                #variants
            }
        }

        impl #enum_name {
            #variants2
        }
    }
    .into()
}
