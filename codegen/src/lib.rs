extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Result, Token};

struct DomainDef {
    canrun_internal: bool,
    domain_visibility: syn::Visibility,
    domain_name: syn::Ident,
    domain_types: Vec<syn::Type>,
}

mod kw {
    syn::custom_keyword!(domain);
}

struct DomainDefs {
    defs: Vec<DomainDef>,
}
impl Parse for DomainDefs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut defs = Vec::new();
        while !input.is_empty() {
            if input.peek(kw::domain) || input.peek2(kw::domain) {
                defs.push(input.parse()?);
            }
        }
        Ok(DomainDefs { defs })
    }
}

impl Parse for DomainDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let domain_visibility = input.parse()?;

        input.parse::<kw::domain>()?;

        let domain_name: syn::Ident = input.parse()?;

        let content;
        syn::braced!(content in input);

        let raw_types: Punctuated<syn::Type, Token![,]> =
            content.parse_terminated(syn::Type::parse)?;
        let domain_types: Vec<_> = raw_types.into_iter().collect();

        Ok(DomainDef {
            canrun_internal: false,
            domain_visibility,
            domain_name,
            domain_types,
        })
    }
}

impl quote::ToTokens for DomainDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DomainDef {
            canrun_internal,
            domain_visibility,
            domain_name,
            domain_types,
        } = self;

        let canrun_mod = if *canrun_internal {
            format_ident!("crate")
        } else {
            format_ident!("canrun")
        };

        let (fields, variants): (Vec<_>, Vec<_>) = (0..domain_types.len())
            .map(|n| (format_ident!("t{}", n), format_ident!("T{}", n)))
            .unzip();

        let value_name = format_ident!("{}Value", domain_name);

        let result = quote! {
            #[derive(std::fmt::Debug)]
            #domain_visibility struct #domain_name {
                #(#fields: #canrun_mod::state::HashMap<#canrun_mod::value::LVar<#domain_types>, #canrun_mod::value::Val<#domain_types>>),*
            }

            impl<'a> #canrun_mod::domains::Domain<'a> for #domain_name {
                type Value = #value_name;
                fn new() -> Self {
                    #domain_name {
                        #(#fields: #canrun_mod::state::HashMap::new(),)*
                    }
                }
                fn unify_domain_values(
                    state: #canrun_mod::state::State<'a, Self>,
                    a: Self::Value,
                    b: Self::Value,
                ) -> Option<#canrun_mod::state::State<Self>> {
                    use #canrun_mod::value::{Val, IntoVal};
                    match (a, b) {
                        #(
                            (#value_name::#variants(a), #value_name::#variants(b)) => {
                                state.unify::<#domain_types>(&a.into_val(), &b.into_val())
                            }
                        ,)*
                        _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                    }
                }
            }

            #(
                impl<'a> #canrun_mod::domains::IntoDomainVal<'a, #domain_types> for #domain_name {
                    fn into_domain_val(val: #canrun_mod::value::Val<#domain_types>) -> #value_name {
                        #value_name::#variants(val)
                    }
                }
            )*

            #(
                impl<'a> #canrun_mod::domains::DomainType<'a, #domain_types> for #domain_name {
                    fn values_as_ref(
                        &self,
                    ) -> &#canrun_mod::state::HashMap<#canrun_mod::value::LVar<#domain_types>, #canrun_mod::value::Val<#domain_types>> {
                        &self.#fields
                    }
                    fn values_as_mut(
                        &mut self,
                    ) -> &mut #canrun_mod::state::HashMap<#canrun_mod::value::LVar<#domain_types>, #canrun_mod::value::Val<#domain_types>> {
                        &mut self.#fields
                    }
                }
            )*

            impl<'a> Clone for #domain_name {
                fn clone(&self) -> Self {
                    #domain_name {
                        #(#fields: self.#fields.clone()),*
                    }
                }
            }

            #[doc(hidden)]
            #[derive(std::fmt::Debug)]
            #domain_visibility enum #value_name {
                #(#variants(#canrun_mod::value::Val<#domain_types>)),*
            }

            impl Clone for #value_name {
                fn clone(&self) -> Self {
                    match self {
                        #(#value_name::#variants(val) => #value_name::#variants(val.clone())),*
                    }
                }
            }
        };
        result.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn domains(item: TokenStream) -> TokenStream {
    let DomainDefs { defs } = parse_macro_input!(item as DomainDefs);
    quote!(#(#defs)*).into()
}

#[proc_macro]
pub fn canrun_internal_domains(item: TokenStream) -> TokenStream {
    let DomainDefs { defs } = parse_macro_input!(item as DomainDefs);
    let defs = defs.into_iter().map(|mut domain: DomainDef| {
        domain.canrun_internal = true;
        domain
    });
    quote!(#(#defs)*).into()
}
