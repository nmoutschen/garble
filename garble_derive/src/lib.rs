//! This crate provides Garble derive macro.
//!
//! ```rust
//! use garble::Garble;
//!
//! #[derive(Garble)]
//! struct MyStruct {
//!     a: u32,
//!     #[nogarble]
//!     b: u32,
//! }
//! ```
//!

use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;
use synstructure::{decl_derive, AddBounds, BindStyle, Structure};

// TODO: Add support for unions

#[derive(Default)]
struct BindingProps {
    nogarble: bool,
}

fn derive_garble(mut s: Structure) -> TokenStream {
    let ast = s.ast();

    s.bind_with(|_bi| BindStyle::Move);

    // Generate function body
    let body = s.each_variant(|vi| {
        let name = vi.ast().ident;

        let mut counter = 0;
        let bodies = vi
            .bindings()
            .iter()
            .map(|bi| {
                let mut props = BindingProps::default();

                for attr in &bi.ast().attrs {
                    if attr.path.is_ident("nogarble") {
                        props.nogarble = true;
                    }
                }

                let ident = &bi.ast().ident;

                let c = syn::Index::from(counter);

                let ret = if props.nogarble {
                    // If we shouldn't garble this field
                    match ident {
                        // If it has an ident
                        Some(i) => quote! {
                            #i: #bi
                        },
                        // If not
                        None => quote! {
                            #c: #bi
                        },
                    }
                } else {
                    match ident {
                        Some(i) => quote! {
                            #i: garbler.garble(#bi)
                        },
                        None => quote! {
                            #c: garbler.garble(#bi)
                        },
                    }
                };
                counter += 1;
                ret
            })
            .collect::<Vec<_>>();

        match s.ast().data {
            Data::Struct(_) => quote! {
                #name { #(#bodies),* }
            },
            Data::Enum(_) => quote! {
                Self::#name { #(#bodies),* }
            },
            Data::Union(_) => quote! {
                #name { #(#bodies),* }
            },
        }
    });

    // Get trait bounds
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let mut where_clause = where_clause.cloned();
    let dummy_const: syn::Ident =
        syn::parse_str(&format!("_DERIVE_garble_Garble_g_FOR_{}", name)).unwrap();
    s.add_trait_bounds(
        &syn::parse_quote!(::garble::Garble<Output = T>),
        &mut where_clause,
        AddBounds::Generics,
    );

    quote! {
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            impl #impl_generics ::garble::Garble for #name #ty_generics #where_clause {
                type Output = Self;

                fn garble<G>(self, garbler: &mut G) -> Self
                where
                    G: ::garble::Garbler
                {
                    match self { #body }
                }
            }
        };
    }
}

decl_derive!([Garble, attributes(nogarble)] => derive_garble);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        synstructure::test_derive! {
            derive_garble {
                enum TestEnum {
                    A { a: u32, b: u32 },
                    B(u32),
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_garble_Garble_g_FOR_TestEnum : () = {
                    impl ::garble::Garble for TestEnum {
                        type Output = Self;
                        fn garble<G> (self, garbler: &mut G) -> Self
                        where G: ::garble::Garbler  {
                            match self {
                                TestEnum::A {a: __binding_0, b: __binding_1,}=> {
                                    Self::A {
                                        a : garbler.garble (__binding_0),
                                        b : garbler.garble (__binding_1)
                                    }
                                }
                                TestEnum::B (__binding_0,) => {
                                    Self::B {
                                        0: garbler.garble(__binding_0)
                                    }
                                }
                            }
                        }
                    }
                };
            }
        };
    }

    #[test]
    fn test_struct() {
        synstructure::test_derive! {
            derive_garble {
                struct MyStruct {
                    a: u32,
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_garble_Garble_g_FOR_MyStruct : () = {
                    impl ::garble::Garble for MyStruct {
                        type Output = Self;
                        fn garble<G>(self, garbler: & mut G)-> Self where G: ::garble::Garbler {
                            match self {
                                MyStruct { a : __binding_0, } => {
                                    MyStruct {
                                        a: garbler.garble(__binding_0)
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
    }

    #[test]
    fn test_struct2() {
        synstructure::test_derive! {
            derive_garble {
                struct MyStruct {
                    a: u32,
                    b: u32,
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_garble_Garble_g_FOR_MyStruct : () = {
                    impl ::garble::Garble for MyStruct {
                        type Output = Self;
                        fn garble<G>(self, garbler: & mut G)-> Self where G: ::garble::Garbler {
                            match self {
                                MyStruct { a : __binding_0, b : __binding_1, } => {
                                    MyStruct {
                                        a: garbler.garble(__binding_0),
                                        b: garbler.garble(__binding_1)
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
    }

    #[test]
    fn test_struct_generic() {
        synstructure::test_derive! {
            derive_garble {
                struct MyStruct<T> {
                    a: T,
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                const _DERIVE_garble_Garble_g_FOR_MyStruct : () = {
                    impl<T> ::garble::Garble for MyStruct<T>
                    where
                        T: ::garble::Garble<Output = T>
                    {
                        type Output = Self;
                        fn garble<G>(self, garbler: & mut G)-> Self where G: ::garble::Garbler {
                            match self {
                                MyStruct { a : __binding_0, } => {
                                    MyStruct {
                                        a: garbler.garble(__binding_0)
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
    }
}
