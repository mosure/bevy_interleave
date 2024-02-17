use quote::quote;
use syn::{
    Data,
    DeriveInput,
    Error,
    Fields,
    FieldsNamed,
    Result,
};


pub fn generate_min_binding_sizes(input: &DeriveInput) -> Result<quote::__private::TokenStream> {
    let name = &input.ident;

    let fields_struct = if let Data::Struct(ref data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(ref fields) => fields,
            _ => return Err(Error::new_spanned(input, "Unsupported struct type")),
        }
    } else {
        return Err(Error::new_spanned(input, "Planar macro only supports structs"));
    };

    let min_binding_size_method = generate_min_binding_size_method(fields_struct);

    let expanded = quote! {
        impl MinBindingSize for #name {
            type PackedType = #name;

            #min_binding_size_method
        }
    };

    Ok(expanded)
}


pub fn generate_min_binding_size_method(fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let min_binding_sizes = fields_named.named
        .iter()
        .map(|f| {
            let field_type = &f.ty;
            quote! {
                std::mem::size_of::<#field_type>()
            }
        });

    quote! {
        fn min_binding_sizes() -> &'static [usize] {
            &[#(#min_binding_sizes),*]
        }
    }
}
