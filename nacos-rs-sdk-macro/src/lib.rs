use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::FromIterator;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Ident, Type};

#[proc_macro_derive(Value)]
pub fn value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    if let Data::Struct(r#struct) = input.data {
        let fields = r#struct.fields;
        if matches!(&fields, Fields::Named(_)) {
            if matches!(&fields, Fields::Named(_)) {
                let builder_set_fields = map_value_fields(&fields, |(ident, sets, ty)| {
                    if ident == "nacos" {
                        return quote!();
                    }
                    if let Some(to) = get_optional_inner_type(ty) {
                        quote!(
                            pub fn #sets(&mut self, value: &#to) {
                                self.#ident = Some(value.clone());
                            }

                            pub fn #ident(&self) -> #ty {
                                self.#ident.clone()
                            }
                        )
                    } else {
                        quote!(
                            pub fn #sets(&mut self, value: &#ty) {
                                self.#ident = value.clone();
                            }

                            pub fn #ident(&self) -> #ty {
                                self.#ident.clone()
                            }
                        )
                    }
                });
                let result = quote!(
                impl #ident {
                    #builder_set_fields
                }
                )
                .into();
                return result;
            }
        }
    }
    quote!().into()
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());
    if let Data::Struct(r#struct) = input.data {
        let fields = r#struct.fields;
        if matches!(&fields, Fields::Named(_)) {
            let builder_fields = map_fields(&fields, |(ident, ty)| {
                if let Some(_) = get_optional_inner_type(ty) {
                    quote!(#ident: #ty, )
                } else {
                    quote!(#ident: Option<#ty>, )
                }
            });
            let builder_set_fields = map_fields(&fields, |(ident, ty)| {
                if let Some(to) = get_optional_inner_type(ty) {
                    quote!(pub fn #ident(mut self, value: #to) -> Self {
                        self.#ident = Some(value);
                        self
                    })
                } else {
                    quote!(pub fn #ident(mut self, value: #ty) -> Self {
                        self.#ident = Some(value);
                        self
                    })
                }
            });
            let builder_token_stream = map_fields(&fields, |(ident, ty)| {
                if let Some(_) = get_optional_inner_type(ty) {
                    quote!(
                        let #ident = self.#ident;
                    )
                } else {
                    quote!(
                        let #ident = self.#ident.ok_or(
                            format!("field \"{}\" required, but not set yet.",stringify!(#ident))
                            )?;
                    )
                }
            });
            let build_values = map_fields(&fields, |(ident, _)| quote!(#ident,));
            let result = quote!(
                impl #ident {
                    pub fn builder() -> #ident_builder {
                        #ident_builder::default()
                    }
                }

                #[derive(Default)]
                pub struct #ident_builder {
                    #builder_fields
                }

                impl #ident_builder {
                    #builder_set_fields

                    pub fn build(self) -> Result<#ident, String> {
                        #builder_token_stream

                        Ok(#ident { #build_values })
                    }
                }
            ).into();
            return result;
        }
    }
    quote!().into()
}
#[proc_macro_derive(Nacos)]
pub fn nacos_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Get));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! (
        impl #impl_generics Nacos for #name #ty_generics #where_clause {
            fn get_token(&self) -> String {
                match self.nacos.clone() {
                    Some(n) => n.read().unwrap().clone().token.unwrap_or("".to_string()),
                    None => "".to_string(),
                }
            }
            fn get_nacos(&self) -> NacosClient {
                self.nacos.clone().unwrap().read().unwrap().clone()
            }
            fn clone_nacos(&self) -> Arc<RwLock<NacosClient>> {
                self.nacos.clone().unwrap()
            }
            fn set_nacos(&mut self, nacos: &Arc<RwLock<NacosClient>>) {
                self.nacos = Some(nacos.clone());
            }
        }
    )
    .into()
}

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

    let uri = Ident::new(
        &format!("{}_URI", name.to_string().to_uppercase()),
        Span::call_site(),
    );
    quote! (
        impl #impl_generics Get for #name #ty_generics #where_clause {
            const URI: &'static str = #uri;
        }
    )
    .into()
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

    let uri = Ident::new(
        &format!("{}_URI", name.to_string().to_uppercase()),
        Span::call_site(),
    );
    quote! (
        impl #impl_generics Post for #name #ty_generics #where_clause {
            const URI: &'static str = #uri;
        }
    )
    .into()
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

    let uri = Ident::new(
        &format!("{}_URI", name.to_string().to_uppercase()),
        Span::call_site(),
    );
    quote! (
        impl #impl_generics Put for #name #ty_generics #where_clause {
            const URI: &'static str = #uri;
        }
    )
    .into()
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

    let uri = Ident::new(
        &format!("{}_URI", name.to_string().to_uppercase()),
        Span::call_site(),
    );
    quote! (
        impl #impl_generics Delete for #name #ty_generics #where_clause {
            const URI: &'static str = #uri;
        }
    )
    .into()
}

fn map_fields<F>(fields: &Fields, mapper: F) -> TokenStream2
where
    F: FnMut((&Ident, &Type)) -> TokenStream2,
{
    TokenStream2::from_iter(
        fields
            .iter()
            .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
            .map(mapper),
    )
}

fn map_value_fields<F>(fields: &Fields, mapper: F) -> TokenStream2
where
    F: FnMut((&Ident, Ident, &Type)) -> TokenStream2,
{
    TokenStream2::from_iter(
        fields
            .iter()
            .map(|field| {
                (
                    field.ident.as_ref().unwrap(),
                    Ident::new(
                        &format!("set_{}", field.ident.as_ref().unwrap().to_string()),
                        Span::call_site(),
                    ),
                    &field.ty,
                )
            })
            .map(mapper),
    )
}

fn get_optional_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        // 这里我们取segments的最后一节来判断是不是`Option<T>`，这样如果用户写的是`std:option:Option<T>`我们也能识别出最后的`Option<T>`
        if let Some(seg) = path.segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
