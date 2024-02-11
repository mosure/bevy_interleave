extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data,
    DeriveInput,
    Error,
    Fields,
    FieldsNamed,
    Ident,
    Result,
    parse_macro_input,
};


#[proc_macro_derive(Planar)]
pub fn planar_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = match generate_planar_struct(&input) {
        Ok(quote) => quote,
        Err(e) => return e.to_compile_error().into(),
    };

    TokenStream::from(output)
}

fn generate_planar_struct(input: &DeriveInput) -> Result<quote::__private::TokenStream> {
    let name = &input.ident;
    let planar_name = Ident::new(&format!("Planar{}", name), name.span());

    let fields_struct = if let Data::Struct(ref data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(ref fields) => fields,
            _ => return Err(Error::new_spanned(&input, "Unsupported struct type")),
        }
    } else {
        return Err(Error::new_spanned(&input, "Planar macro only supports structs"));
    };

    let field_names = fields_struct.named.iter().map(|f| f.ident.as_ref().unwrap());
    let field_types = fields_struct.named.iter().map(|_| quote! { Vec<i32> });

    let conversion_methods = generate_conversion_methods(name, fields_struct);
    let get_set_methods = generate_accessor_setter_methods(name, fields_struct);
    let len_method = generate_len_method(fields_struct);

    let expanded = quote! {
        pub struct #planar_name {
            #(pub #field_names: #field_types,)*
        }

        impl Planar<#name> for #planar_name {
            #conversion_methods
            #get_set_methods
            #len_method
        }
    };

    Ok(expanded)
}


fn generate_len_method(fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    if let Some(first_field) = fields_named.named.first() {
        let first_field_name = first_field.ident.as_ref().unwrap();
        quote! {
            fn len(&self) -> usize {
                self.#first_field_name.len()
            }
        }
    } else {
        quote! {
            fn len(&self) -> usize {
                0
            }
        }
    }
}


fn generate_accessor_setter_methods(struct_name: &Ident, fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let packed_assignments = fields_named.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! { #name: self.#name[index].clone() }
    });

    let set_assignments = fields_named.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! { self.#name[index] = value.#name.clone(); }
    });

    quote! {
        fn get(&self, index: usize) -> #struct_name {
            #struct_name {
                #(#packed_assignments),*
            }
        }

        fn set(&mut self, index: usize, value: #struct_name) {
            #(#set_assignments);*
        }
    }
}


fn generate_conversion_methods(struct_name: &Ident, fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let (
        from_interleaved_fields,
        to_interleaved_fields_templates
    ): (Vec<_>, Vec<_>) = fields_named.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();

        let from_interleaved_field = quote! {
            #name: packed.iter().map(|x| x.#name.clone()).collect()
        };
        let to_interleaved_field_template = quote! {
            #name: self.#name[index].clone()
        };

        (from_interleaved_field, to_interleaved_field_template)
    }).unzip();

    let to_interleaved_method = quote! {
        fn to_interleaved(&self) -> Vec<#struct_name> {
            (0..self.len())
                .map(|index| #struct_name {
                    #(#to_interleaved_fields_templates),*
                })
                .collect()
        }
    };

    let conversion_methods = quote! {
        fn from_interleaved(packed: Vec<#struct_name>) -> Self {
            Self {
                #(#from_interleaved_fields),*
            }
        }
        #to_interleaved_method
    };

    conversion_methods
}
