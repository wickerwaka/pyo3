// Copyright (c) 2017-present PyO3 Project and Contributors

use crate::defs;
use crate::func::{impl_method_proto, MethodProto};
use crate::method::FnSpec;
use crate::pymethod;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn build_py_proto(ast: &mut syn::ItemImpl) -> syn::Result<TokenStream> {
    if let Some((_, ref mut path, _)) = ast.trait_ {
        let proto = if let Some(ref mut segment) = path.segments.last() {
            match defs::find_protocol(segment.ident.to_string().as_str()) {
                Some(p) => p,
                None => {
                    return Err(syn::Error::new_spanned(
                        path,
                        "#[pyproto] can not be used with this block",
                    ));
                }
            }
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "#[pyproto] can only be used with protocol trait implementations",
            ));
        };

        let tokens = impl_proto_impl(&ast.self_ty, &mut ast.items, proto);

        // attach lifetime
        let mut seg = path.segments.pop().unwrap().into_value();
        seg.arguments = syn::PathArguments::AngleBracketed(syn::parse_quote! {<'p>});
        path.segments.push(seg);
        ast.generics.params = syn::parse_quote! {'p};

        Ok(tokens)
    } else {
        Err(syn::Error::new_spanned(
            ast,
            "#[pyproto] can only be used with protocol trait implementations",
        ))
    }
}

fn impl_proto_impl(
    ty: &syn::Type,
    impls: &mut Vec<syn::ImplItem>,
    proto: &defs::Proto,
) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut py_methods = Vec::new();

    let mut unimpl_methods: Vec<&MethodProto> = proto.methods.iter().collect();
    let mut unimpl_py_methods: Vec<&defs::PyMethod> = proto.py_methods.iter().collect();

    for iimpl in impls.iter_mut() {
        if let syn::ImplItem::Method(ref mut met) = iimpl {
            let method_name = met.sig.ident.to_string();
            // Find the method in unimpl_methods, remove it and implement it
            unimpl_methods
                .iter()
                .position(|m| *m == method_name.as_str())
                .map(|idx| {
                    let method = unimpl_methods.swap_remove(idx);
                    impl_method_proto(ty, &mut met.sig, method).to_tokens(&mut tokens);
                });

            // MJDFIXME - undo this change
            let py_method_name = &met.sig.ident;
            let method = unimpl_py_methods
                .iter()
                .position(|m| m.name == py_method_name.to_string().as_str())
                .map(|idx| unimpl_py_methods.swap_remove(idx));

            if let Some(m) = method {
                let name = syn::Ident::new(m.name, Span::call_site());
                let proto: syn::Path = syn::parse_str(m.proto).unwrap();

                let fn_spec = match FnSpec::parse(&py_method_name, &met.sig, &mut met.attrs) {
                    Ok(fn_spec) => fn_spec,
                    Err(err) => return err.to_compile_error(),
                };
                let meth = pymethod::impl_proto_wrap(ty, &py_method_name, &fn_spec);

                py_methods.push(quote! {
                    impl #proto for #ty
                    {
                        #[inline]
                        fn #name() -> Option<pyo3::class::methods::PyMethodDef> {
                            #meth

                            Some(pyo3::class::PyMethodDef {
                                ml_name: stringify!(#name),
                                ml_meth: pyo3::class::PyMethodType::PyCFunctionWithKeywords(__wrap),
                                ml_flags: pyo3::ffi::METH_VARARGS | pyo3::ffi::METH_KEYWORDS,
                                ml_doc: ""
                            })
                        }
                    }
                });
            };
        }
    }

    let default_impls: Vec<_> = unimpl_methods
        .iter()
        .map(|m| {
            let proto: syn::Path = syn::parse_str(m.get_default()).unwrap();
            quote! { impl #proto for #ty {} }
        })
        .collect();
    quote! {
        #tokens

        #(#py_methods)*

        #(#default_impls)*
    }
}
