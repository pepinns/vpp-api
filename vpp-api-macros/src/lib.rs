extern crate proc_macro;
use quote::quote;
use syn;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

use proc_macro2::TokenTree;

#[proc_macro_derive(VppMessage, attributes(message_name_and_crc))]
pub fn derive_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attribute_tokens = input.attrs[0].tokens.clone();
    let mut token_iter = attribute_tokens.into_iter();
    let first = token_iter.next().unwrap();
    let ident = match first {
        TokenTree::Group(ref g) => {
            let stream = g.stream().clone();
            let mut stream_iter = stream.into_iter();
            stream_iter.next().unwrap().to_string()
        }
        _ => panic!("Wrong format for message name and crc"),
    };
    let name = input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!();
    };
    let option_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: std::option::Option<#ty>}
    });
    let builder_init = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name: None}
    });
    let field_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, #name:#ty) -> &mut Self{
                self.#name = Some(#name);
                self
            }
        }
    });
    let field_methods_trait = fields.iter().filter_map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if let Some(nn) = name {
            if nn.to_string() == "client_index".to_string()
                || nn.to_string() == "context".to_string()
            {
                let fn_name = syn::Ident::new(&format!("with_{}", nn.to_string()), f.ident.span());
                return Some(quote! {
                    fn #fn_name(&mut self, #name:#ty) -> &mut Self{
                        self.#name = Some(#name);
                        self
                    }
                });
            }
        }
        None
    });
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if let Some(nn) = name {
            if nn.to_string() == "client_index".to_string() || nn.to_string() == "context".to_string() {
                return quote! {
                    #name: self.#name.clone().unwrap_or_default()
                };
            }
        }
        quote! {
            #name: self.#name.clone().ok_or(concat!("Message Field: '" , stringify!(#name), "' is not set"))?
        }
    });
    let build_fields_require = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.clone().ok_or(concat!("Message Field: '" , stringify!(#name), "' is not set"))?
        }
    });

    let setters = fields.iter().filter_map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if let Some(nn) = &f.ident {
            let fn_name = syn::Ident::new(&format!("set_{}", nn.to_string()), f.ident.span());
            if nn.to_string() == "client_index".to_string()
                || nn.to_string() == "context".to_string()
            {
                return Some(quote! {
                    fn #fn_name(&mut self, #name: #ty) {
                        self.#name = #name;
                    }
                });
            }
        }
        None
    });

    // only want to setup the setters if the request has both.
    let setters2: Vec<()> = fields
        .iter()
        .filter_map(|f| {
            let name = &f.ident;
            if let Some(nn) = name {
                if nn.to_string() == "client_index".to_string()
                    || nn.to_string() == "context".to_string()
                {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let builder_ident = syn::Ident::new(&format!("Builder{}", name.to_string()), name.span());
    let request = if setters2.len() > 1 {
        quote! {
            impl vpp_api_message::VppApiRequest for #name {
                #(#setters)*
            }
            impl vpp_api_message::VppApiRequestBuilder for #builder_ident {
                type Item = #name;

                #(#field_methods_trait)*
                fn build(&mut self) -> Result<Self::Item, Box<dyn std::error::Error>>{
                    Ok(#name{
                        #(#build_fields_require,)*
                    })
                }
            }
        }
    } else {
        quote! {
            impl vpp_api_message::VppApiResponse for #name {

            }
        }
    };

    let expanded = quote! {
         #[derive(Debug,Clone)]
         pub struct #builder_ident{
             #(#option_fields,)*
         }
         impl #builder_ident{
             #(#field_methods)*
             pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>>{
                 Ok(#name{
                     #(#build_fields,)*
                })
             }
         }

         #request

         impl VppApiMessage for #name {
            fn get_message_name_and_crc() -> String {
                 String::from(#ident)
            }
         }
         impl #name {
            pub fn get_message_name_and_crc() -> String {
                 String::from(#ident)
            }
            pub fn builder() -> #builder_ident{
                #builder_ident{
                 #(#builder_init,)*
                }
            }
        }
    };
    expanded.into()
}
#[proc_macro_derive(VppUnionIdent, attributes(types))]
pub fn derive_unionident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty: &syn::Ident;
    match input.data {
        syn::Data::Struct(ref ds) => {
            match ds.fields {
                syn::Fields::Unnamed(ref fu) => {
                    match fu.unnamed.first().unwrap().ty {
                        syn::Type::Path(ref tp) => {
                            // eprintln!("{:#?}", tp.path.segments[0].arguments);
                            match tp.path.segments[0].arguments {
                                syn::PathArguments::AngleBracketed(ref arg) => {
                                    // eprintln!("{:#?}", arg.args[0]);
                                    // eprintln!("{:#?}", arg.args);
                                    match arg.args[1] {
                                        syn::GenericArgument::Type(ref typt) => {
                                            match typt {
                                                syn::Type::Path(ref typath) => {
                                                    // eprintln!("{:#?}", typath.path.segments[1].ident);
                                                    ty = &typath.path.segments[1].ident;
                                                }
                                                _ => panic!("Wrong input"),
                                            }
                                        }
                                        _ => panic!("Wrong Input"),
                                    }
                                }
                                _ => panic!("Wrong input"),
                            }
                        }
                        _ => panic!("Wrong Input"),
                    }
                }
                _ => panic!("Named fields"),
            }
        }
        _ => panic!("Wrong data structure"),
    }
    let maxsize = ty.clone().to_string().trim_start_matches("U").to_string();
    let maxsize_literal = syn::LitInt::new(&maxsize, ty.span());
    let name = input.ident;
    let helperfunctions = input.attrs.iter().map(|f| {
        let mut group_stream = f.tokens.clone().into_iter();
        let stream_group = group_stream.next().unwrap();
        let ident;
        let liter;
        match stream_group {
            TokenTree::Group(ref g) => {
                let mut iterterator = g.stream().into_iter();
                ident = iterterator.next().unwrap();
                let _punt = iterterator.next().unwrap();
                liter = iterterator.next().unwrap();
            }
            _ => panic!("Felix! Something went wrong"),
        }
        let function_name_new = format!("new_{}", ident.to_string());
        let function_name_new_ident = syn::Ident::new(&function_name_new, name.span());
        let _function_name_set_ident =
            syn::Ident::new(&format!("set_{}", ident.to_string()), name.span());
        let function_name_get_ident =
            syn::Ident::new(&format!("get_{}", ident.to_string()), name.span());
        quote! {
                pub fn #function_name_new_ident(some: #ident) -> #name{
                    let mut arr: Vec<u8> = vec![0;#maxsize_literal];
                    let some_arr: Vec<u8> = bincode::serialize(&some).unwrap();
                    for x in 0..#liter{
                        arr[x] = some_arr[x];
                    }
                    #name(arr.try_into().unwrap())
                 }
               /* pub fn #function_name_set_ident(&mut self, some:#ident){
                    let mut some_arr: Vec<u8> = bincode::serialize(&some).unwrap();
                    self.0.0[0..#liter].clone_from_slice(&some);
                } */
                pub fn #function_name_get_ident(&self) -> #ident{
                    let some = self.0.0.clone();
                    let mut someIdent: Vec<u8> = vec![0;#liter];
                    someIdent.clone_from_slice(&some[0..#liter]);
                    let decoded: #ident = bincode::deserialize(&someIdent).unwrap();
                    decoded
                }
        }
    });
    let expanded = quote! {
        // use bincode;
        impl #name{
            fn new() -> #name {
                let mut out: FixedSizeArray<u8, typenum::#ty> = Default::default();
                #name(out)
            }
            #(#helperfunctions)*
        }
    };
    expanded.into()
}
