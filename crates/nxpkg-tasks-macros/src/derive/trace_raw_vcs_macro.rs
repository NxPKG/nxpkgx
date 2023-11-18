use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, FieldsUnnamed};
use nxpkg_tasks_macros_shared::{generate_destructuring, match_expansion};

use super::FieldAttributes;

fn filter_field(field: &Field) -> bool {
    !FieldAttributes::from(field.attrs.as_slice()).trace_ignore
}

pub fn derive_trace_raw_vcs(input: TokenStream) -> TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;

    for type_param in derive_input.generics.type_params_mut() {
        type_param
            .bounds
            .push(syn::parse_quote!(nxpkg_tasks::trace::TraceRawVcs));
    }
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let trace_items = match_expansion(&derive_input, &trace_named, &trace_unnamed, &trace_unit);
    quote! {
        impl #impl_generics nxpkg_tasks::trace::TraceRawVcs for #ident #ty_generics #where_clause {
            fn trace_raw_vcs(&self, __context__: &mut nxpkg_tasks::trace::TraceRawVcsContext) {
                #trace_items
            }
        }
    }
    .into()
}

fn trace_named(_ident: &Ident, fields: &FieldsNamed) -> (TokenStream2, TokenStream2) {
    let (captures, fields_idents) = generate_destructuring(fields.named.iter(), &filter_field);
    (
        captures,
        quote! {
            {#(
                nxpkg_tasks::trace::TraceRawVcs::trace_raw_vcs(#fields_idents, __context__);
            )*}
        },
    )
}

fn trace_unnamed(_ident: &Ident, fields: &FieldsUnnamed) -> (TokenStream2, TokenStream2) {
    let (captures, fields_idents) = generate_destructuring(fields.unnamed.iter(), &filter_field);
    (
        captures,
        quote! {
            {#(
                nxpkg_tasks::trace::TraceRawVcs::trace_raw_vcs(#fields_idents, __context__);
            )*}
        },
    )
}

fn trace_unit(_ident: &Ident) -> (TokenStream2, TokenStream2) {
    (quote! {}, quote! { { } })
}
