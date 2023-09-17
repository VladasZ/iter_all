use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(IterAll)]
pub fn iter_all_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let Data::Enum(ref data_enum) = input.data else {
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

    let variants3 = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let Fields::Unnamed(ref unnamed) = variant.fields else {
            panic!();
        };

        let ty: &Type = &unnamed.unnamed.first().unwrap().ty;

        let ty_str = ty.to_owned().to_token_stream().to_string();

        let start = ty_str.find('<').unwrap();
        let end = ty_str.find('>').unwrap();

        let generics = &ty_str[start + 1..end].trim();

        let only_type = &ty_str[0..start - 1].trim();

        let uppercase_ident = format_ident!("{}", variant_name.to_string().to_uppercase());
        let only_type = format_ident!("{}", only_type);

        use std::str::FromStr;

        let generics: proc_macro2::TokenStream = TokenStream::from_str(generics).unwrap().to_owned().into();

        quote! {
            pub const #uppercase_ident: #ty = #only_type::<#generics>::new();
        }
    });

    let variants3 = quote! {
        #(#variants3)*
    };

    quote! {
        impl iter_all::IterAll for #enum_name {
            fn iter_all(mut action: impl FnMut(Self)) {
                #variants
            }
        }

        impl #enum_name {
            #variants3
        }
    }
    .into()
}
