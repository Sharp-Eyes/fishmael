use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, spanned::Spanned, Data, DeriveInput, Fields};


#[proc_macro_derive(RedisFieldProvider)]
pub fn derive_cacheable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);

    let name = derive_input.ident;

    let derive_impl = make_impl(&derive_input.data);

    let expanded = quote! {
        impl ::fishmael_cache_core::RedisFieldProvider for #name {
            fn add_fields_to_cmd(self, cmd: &mut redis::Cmd) {
                #derive_impl
            }
        }
    };

    expanded.into()
}


fn make_impl(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref inner_data) => {
            match inner_data.fields {
                Fields::Named(ref fields) => {
                    let body = fields.named.iter().map(|field| {
                        let name = &field.ident;
                        let name_str = name.as_ref().map(|n| n.to_string());
                        quote_spanned! {
                            field.span()=>::fishmael_cache_core::ToRedisHArgs::write_redis_hargs(&self.#name, #name_str, cmd);
                        }
                    });

                    quote!(#(#body)*)
                },
                Fields::Unnamed(_) | Fields::Unit => unimplemented!()
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!()
    }
}