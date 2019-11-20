extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use syn::Data;
use syn::Type;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, Ident};

const PRIMITIVES: [&'static str; 15] = [
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "bool",
    "usize", "isize",
];

/// A procedural macro for automatically deriving a C-FFI for a struct
//
/// # Example
/// ```rust
/// # use effy_derive::FFI;
/// #[derive(Default, FFI)]
/// struct MyStruct {
///     field: u32,
/// }
///
/// fn test() {
///     unsafe {
///         let s: *mut MyStruct = my_struct_new();
///
///         my_struct_set_field(s, 15);
///         assert_eq!(my_struct_field(s), 15);
///
///         my_struct_free(s);
///     }
/// }
/// ```
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

    let ident = &input.ident;

    // Create a type alias from our original struct for safer opaque pointers
    // E.g. MyLongStruct -> my_long_struct_t
    let mut ident_alias = String::from(&name);
    ident_alias.push_str("_t");
    let ident_alias = Ident::new(&ident_alias, ident.span());

    // Derive the various functions
    let new_free = derive_new_free(&name, &ident_alias);
    let methods = match &input.data {
        Data::Struct(data) => derive_struct(&name, &ident_alias, &data),
        _ => unimplemented!(),
    };

    // TODO: Get docstring from original struct and copy to alias?

    let output = quote! {
        #[allow(non_camel_case_types)]
        pub type #ident_alias = #ident;

        #new_free
        #methods
    };

    output.into()
}

fn derive_new_free(name: &str, ident: &Ident) -> TokenStream {
    let new_ident = format_ident!("{}_new", name);
    let free_ident = format_ident!("{}_free", name);

    quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #new_ident() -> *mut #ident {
            Box::into_raw(Box::new(#ident::default()))
        }

        #[no_mangle]
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
        let ty_ident_string = ty_ident.to_string();
        let field_ident = &field.ident.as_ref().unwrap();

        if PRIMITIVES.contains(&ty_ident_string.as_str()) {
            functions.push(derive_primitive(name, ident, field_ident, ty));
        }

        if ty_ident_string == "String" {
            functions.push(derive_string(name, ident, field_ident));
            // TODO: Derive String
            // TODO: Handle str?
        }

        // TODO: Derive Vec
        // TODO: Derive Enum
        // TODO: Derive Array
        // TODO: Derive Tuple
    }
    quote! {#(#functions)*}
}

fn derive_primitive(name: &str, ident: &Ident, field: &Ident, ty: &syn::Path) -> TokenStream {
    let get_ident = format_ident!("{}_{}", name, &field);
    let set_ident = format_ident!("{}_set_{}", name, &field);

    let name = Ident::new(name, ident.span());

    quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #get_ident(#name: *mut #ident) -> #ty {
            let #name: Box<#ident> = Box::from_raw(#name);
            let field = #name.#field;
            std::mem::forget(#name);
            field
        }

        #[no_mangle]
        pub unsafe extern "C" fn #set_ident(#name: *mut #ident, val: #ty) {
            let mut #name: Box<#ident> = Box::from_raw(#name);
            #name.#field = val;
            std::mem::forget(#name);
        }
    }
}

fn derive_string(name: &str, ident: &Ident, field: &Ident) -> TokenStream {
    let get_ident = format_ident!("{}_{}", name, &field);
    let set_ident = format_ident!("{}_set_{}", name, &field);

    let name = Ident::new(name, ident.span());

    quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #get_ident(#name: *mut #ident, out: *mut string_t) {
            let #name: Box<#ident> = Box::from_raw(#name);
            let mut out = Box::from_raw(out);
            out.set_string(&#name.#field);
            std::mem::forget(out);
            std::mem::forget(#name);
        }

        #[no_mangle]
        pub unsafe extern "C" fn #set_ident(#name: *mut #ident, new_str: *mut c_char) {
            let mut #name = Box::from_raw(#name);
            #name.#field = std::ffi::CStr::from_ptr(new_str).to_string_lossy().to_string();
            std::mem::forget(#name);
        }
    }
}
