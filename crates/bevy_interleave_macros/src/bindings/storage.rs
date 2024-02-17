use convert_case::{
    Case,
    Casing,
};
use quote::quote;
use syn::{
    Data,
    DeriveInput,
    Error,
    Fields,
    FieldsNamed,
    Ident,
    Result,
};


pub fn storage_bindings(input: &DeriveInput) -> Result<quote::__private::TokenStream> {
    let name = &input.ident;

    let planar_name = Ident::new(&format!("Planar{}", name), name.span());
    let gpu_planar_name = Ident::new(&format!("GpuPlanar{}", name), name.span());

    let fields_struct = if let Data::Struct(ref data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(ref fields) => fields,
            _ => return Err(Error::new_spanned(input, "Unsupported struct type")),
        }
    } else {
        return Err(Error::new_spanned(input, "Planar macro only supports structs"));
    };

    let field_names = fields_struct.named.iter().map(|f| f.ident.as_ref().unwrap());
    let field_types = fields_struct.named.iter().map(|_| {
        quote! { bevy::render::render_resource::Buffer }
    });

    let bind_group = generate_bind_group_method(name, fields_struct);
    let bind_group_layout = generate_bind_group_layout_method(name, fields_struct);
    let prepare = generate_prepare_method(fields_struct);

    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct #gpu_planar_name {
            #(pub #field_names: #field_types,)*
        }

        impl GpuStoragePlanar for #gpu_planar_name {
            type PackedType = #name;
            type PlanarType = #planar_name;

            #bind_group
            #bind_group_layout
            #prepare
        }
    };

    Ok(expanded)
}


pub fn generate_bind_group_method(struct_name: &Ident, fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let struct_name_snake = struct_name.to_string().to_case(Case::Snake);
    let bind_group_name = format!("{}_bind_group", struct_name_snake);

    let bind_group_entries = fields_named.named
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let name = field.ident.as_ref().unwrap();
            quote! {
                bevy::render::render_resource::BindGroupEntry {
                    binding: #idx as u32,
                    resource: bevy::render::render_resource::BindingResource::Buffer(
                        bevy::render::render_resource::BufferBinding {
                            buffer: &self.#name,
                            offset: 0,
                            size: bevy::render::render_resource::BufferSize::new(self.#name.size()),
                        }
                    ),
                },
            }
        });

    quote! {
        fn bind_group(
            &self,
            render_device: &bevy::render::renderer::RenderDevice,
            layout: &bevy::render::render_resource::BindGroupLayout,
        ) -> bevy::render::render_resource::BindGroup {
            render_device.create_bind_group(
                #bind_group_name,
                &layout,
                &[
                    #(#bind_group_entries)*
                ]
            )
        }
    }
}


pub fn generate_bind_group_layout_method(struct_name: &Ident, fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let struct_name_snake = struct_name.to_string().to_case(Case::Snake);
    let bind_group_layout_name = format!("{}_bind_group_layout", struct_name_snake);

    let bind_group_layout_entries = fields_named.named
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            quote! {
                bevy::render::render_resource::BindGroupLayoutEntry {
                    binding: #idx as u32,
                    visibility: bevy::render::render_resource::ShaderStages::all(),
                    ty: bevy::render::render_resource::BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Storage { read_only },
                        has_dynamic_offset: false,
                        min_binding_size: bevy::render::render_resource::BufferSize::new(Self::PackedType::min_binding_sizes()[#idx] as u64),
                    },
                    count: None,
                },
            }
        });

    quote! {
        fn bind_group_layout(
            &self,
            render_device: &bevy::render::renderer::RenderDevice,
            read_only: bool,
        ) -> bevy::render::render_resource::BindGroupLayout {
            render_device.create_bind_group_layout(
                &bevy::render::render_resource::BindGroupLayoutDescriptor {
                    label: Some(#bind_group_layout_name),
                    entries: &[
                        #(#bind_group_layout_entries)*
                    ],
                }
            )
        }
    }
}


pub fn generate_prepare_method(fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let buffers = fields_named.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let buffer_name_string = format!("{}_buffer", name);

            quote! {
                let #name = render_device.create_buffer_with_data(
                    &bevy::render::render_resource::BufferInitDescriptor {
                        label: Some(#buffer_name_string),
                        contents: bytemuck::cast_slice(planar.#name.as_slice()),
                        usage: bevy::render::render_resource::BufferUsages::COPY_DST
                             | bevy::render::render_resource::BufferUsages::STORAGE,
                    }
                );
            }
        });

    let buffer_names = fields_named.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote! { #name }
        });

    quote! {
        fn prepare(
            render_device: &bevy::render::renderer::RenderDevice,
            planar: &Self::PlanarType,
        ) -> Self {
            #(#buffers)*

            Self {
                #(#buffer_names),*
            }
        }
    }
}
