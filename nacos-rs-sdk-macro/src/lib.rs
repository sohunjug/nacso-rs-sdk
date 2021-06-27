use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam};

#[proc_macro_derive(Get)]
pub fn get_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Get));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Get for #name #ty_generics #where_clause {}
    })
}

#[proc_macro_derive(Post)]
pub fn post_derive(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Post));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Post for #name #ty_generics #where_clause {}
    })
}

#[proc_macro_derive(Put)]
pub fn put_derive(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Put));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Put for #name #ty_generics #where_clause {}
    })
}

#[proc_macro_derive(Delete)]
pub fn delete_derive(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Delete));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Delete for #name #ty_generics #where_clause {}
    })
}
