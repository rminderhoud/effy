extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use syn::Data;
use syn::Type;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, Ident};

const PRIMITIVES: [&'static str; 13] = [
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "bool",
];

#[proc_macro_derive(FFI)]
pub fn derive_effy(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Convert struct name to lowercase splitting on uppercase characters.
    // E.g. MyLongStructName -> my_long_struct_name
    let mut name = String::new();
    for (idx, c) in input.ident.to_string().char_indices() {
        if idx > 0 {
            if c.is_uppercase() {
                name.push('_');
            }
        }
        for c in c.to_lowercase() {
            name.push(c);
        }
    }

    let new_free = derive_new_free(&name, &input.ident);

    let methods = match input.data {
        Data::Struct(data) => derive_struct(&name, &input.ident, &data),
        _ => unimplemented!(),
    };

    // TODO: Derive primitive
    // TODO: Derive String
    // TODO: Derive Vec
    // TODO: Derive Enum
    // TODO: Derive Array
    // TODO: Derive Tuple

    let output = quote! {
        pub mod FFI {
            use super::*;

            #new_free
            #methods

            pub fn hello() {
                println!("Hello world")
            }
        }
    };

    output.into()
}

fn derive_new_free(name: &str, ident: &Ident) -> TokenStream {
    let new_ident = format_ident!("{}_new", name);
    let free_ident = format_ident!("{}_free", name);

    quote! {
        pub unsafe extern "C" fn #new_ident() -> *mut #ident {
            Box::into_raw(Box::new(#ident::default()))
        }

        pub unsafe extern "C" fn #free_ident(i: *mut #ident) {
            Box::from_raw(i); // Automatically drop
        }
    }
}

fn derive_struct(name: &str, ident: &Ident, data: &DataStruct) -> TokenStream {
    let mut functions = Vec::new();
    for field in data.fields.iter() {
        let ty = match &field.ty {
            Type::Path(path) => &path.path,
            _ => panic!("Unexpected field type in struct"),
        };

        let ty_ident = &ty.segments.last().unwrap().ident;
        let field_ident = &field.ident.as_ref().unwrap();

        if PRIMITIVES.contains(&ty_ident.to_string().as_str()) {
            functions.push(derive_primitive(name, ident, field_ident, ty));
        }
    }
    quote! {#(#functions)*}
}

fn derive_primitive(name: &str, ident: &Ident, field: &Ident, ty: &syn::Path) -> TokenStream {
    let get_ident = format_ident!("{}_{}", name, &field);
    let set_ident = format_ident!("{}_set_{}", name, &field);

    let name = Ident::new(name, ident.span());

    quote! {
        pub unsafe extern "C" fn #get_ident(#name: *mut #ident) -> #ty {
            let #name: Box<#ident> = Box::from_raw(#name);
            let field = #name.#field;
            ::std::mem::forget(#name);
            field
        }

        pub unsafe extern "C" fn #set_ident(#name: *mut #ident, val: #ty) {
            let mut #name: Box<#ident> = Box::from_raw(#name);
            #name.#field = val;
            ::std::mem::forget(#name);
        }
    }
}
