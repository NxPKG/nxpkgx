use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use nxpkg_tasks_macros_shared::{get_type_ident, PrimitiveInput};

use crate::value_macro::value_type_and_register;

pub fn primitive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PrimitiveInput);

    let ty = input.ty;
    let Some(ident) = get_type_ident(&ty) else {
        return quote! {
            // An error occurred while parsing the ident.
        }
        .into();
    };

    let value_debug_impl = quote! {
        #[nxpkg_tasks::value_impl]
        impl nxpkg_tasks::debug::ValueDebug for #ty {
            #[nxpkg_tasks::function]
            async fn dbg(&self) -> anyhow::Result<nxpkg_tasks::Vc<nxpkg_tasks::debug::ValueDebugString>> {
                use nxpkg_tasks::debug::ValueDebugFormat;
                self.value_debug_format(usize::MAX).try_to_value_debug_string().await
            }

            #[nxpkg_tasks::function]
            async fn dbg_depth(&self, depth: usize) -> anyhow::Result<nxpkg_tasks::Vc<nxpkg_tasks::debug::ValueDebugString>> {
                use nxpkg_tasks::debug::ValueDebugFormat;
                self.value_debug_format(depth).try_to_value_debug_string().await
            }
        }
    };

    let value_type_and_register = value_type_and_register(
        &ident,
        quote! { #ty },
        None,
        quote! {
            nxpkg_tasks::VcTransparentRead<#ty, #ty, #ty>
        },
        quote! {
            nxpkg_tasks::VcCellSharedMode<#ty>
        },
        quote! {
            nxpkg_tasks::ValueType::new_with_any_serialization::<#ty>()
        },
    );

    let value_default_impl = quote! {
        #[nxpkg_tasks::value_impl]
        impl nxpkg_tasks::ValueDefault for #ty {
            #[nxpkg_tasks::function]
            fn value_default() -> Vc<Self> {
                Vc::cell(Default::default())
            }
        }
    };

    quote! {
        #value_type_and_register

        #value_debug_impl

        #value_default_impl
    }
    .into()
}
