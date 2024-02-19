use convert_case::{
    Case,
    Casing,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute,
    Data,
    DeriveInput,
    Error,
    Fields,
    FieldsNamed,
    Ident,
    parse::{
        Parse,
        ParseStream,
    },
    Path,
    Result,
};


pub fn texture_bindings(input: &DeriveInput) -> Result<quote::__private::TokenStream> {
    let name = &input.ident;

    let planar_name = Ident::new(&format!("Planar{}", name), name.span());
    let gpu_planar_name = Ident::new(&format!("PlanarTexture{}", name), name.span());

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
        quote! { bevy::asset::Handle<bevy::render::texture::Image> }
    });

    let bind_group = generate_bind_group_method(name, fields_struct);
    let bind_group_layout = generate_bind_group_layout_method(name, fields_struct);
    let prepare = generate_prepare_method(fields_struct);
    let get_asset_handles = generate_get_asset_handles_method(fields_struct);

    let expanded = quote! {
        #[derive(bevy::prelude::Component, Default, Clone, Debug, bevy::reflect::Reflect)]
        pub struct #gpu_planar_name {
            #(pub #field_names: #field_types,)*
        }

        impl bevy::render::extract_component::ExtractComponent for #gpu_planar_name {
            type QueryData = &'static Self;

            type QueryFilter = ();
            type Out = Self;

            fn extract_component(texture_buffers: bevy::ecs::query::QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
                texture_buffers.clone().into()
            }
        }

        impl PlanarTexture for #gpu_planar_name {
            type PackedType = #name;
            type PlanarType = #planar_name;

            #bind_group
            #bind_group_layout
            #prepare
            #get_asset_handles
        }
    };

    Ok(expanded)
}


pub fn generate_bind_group_method(struct_name: &Ident, fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let struct_name_snake = struct_name.to_string().to_case(Case::Snake);
    let bind_group_name = format!("texture_{}_bind_group", struct_name_snake);

    let bind_group_entries = fields_named.named
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let name = field.ident.as_ref().unwrap();
            quote! {
                bevy::render::render_resource::BindGroupEntry {
                    binding: #idx as u32,
                    resource: bevy::render::render_resource::BindingResource::TextureView(
                        &gpu_images.get(&self.#name).unwrap().texture_view
                    ),
                },
            }
        });

    quote! {
        fn bind_group(
            &self,
            render_device: &bevy::render::renderer::RenderDevice,
            gpu_images: &bevy::render::render_asset::RenderAssets<bevy::render::texture::Image>,
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
    let bind_group_layout_name = format!("texture_{}_bind_group_layout", struct_name_snake);

    let bind_group_layout_entries = fields_named.named
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let name = field.ident.as_ref().unwrap();
            let format = extract_texture_format(&field.attrs);

            let field_type = &field.ty;

            quote! {
                let sample_type = #format.sample_type(None, None).unwrap();

                let size = std::mem::size_of::<#field_type>();
                let format_bpp = #format.pixel_size();
                let depth = (size as f32 / format_bpp as f32).ceil() as u32;

                let view_dimension = if depth == 1 {
                    bevy::render::render_resource::TextureViewDimension::D2  // TODO: support 3D texture sampling
                } else {
                    bevy::render::render_resource::TextureViewDimension::D2Array
                };

                let #name = bevy::render::render_resource::BindGroupLayoutEntry {
                    binding: #idx as u32,
                    visibility: bevy::render::render_resource::ShaderStages::all(),
                    ty: bevy::render::render_resource::BindingType::Texture {
                        view_dimension,
                        sample_type,
                        multisampled: false,
                    },
                    count: None,
                };
            }
        });

    let layout_names = fields_named.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote! { #name }
        });

    quote! {
        fn bind_group_layout(
            render_device: &bevy::render::renderer::RenderDevice,
        ) -> bevy::render::render_resource::BindGroupLayout {
            #(#bind_group_layout_entries)*

            render_device.create_bind_group_layout(
                Some(#bind_group_layout_name),
                &[
                    #(#layout_names),*
                ],
            )
        }
    }
}


pub fn generate_prepare_method(fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let buffers = fields_named.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let format = extract_texture_format(&field.attrs);

            let field_type = &field.ty;

            quote! {
                let square = (planar.#name.len() as f32).sqrt().ceil() as u32;

                let size = std::mem::size_of::<#field_type>();
                let format_bpp = #format.pixel_size();
                let depth = (size as f32 / format_bpp as f32).ceil() as u32;

                let mut data = bytemuck::cast_slice(planar.#name.as_slice()).to_vec();

                let padded_size = (square * square * depth * format_bpp as u32) as usize;
                data.resize(padded_size, 0);

                let mut #name = bevy::render::texture::Image::new(
                    bevy::render::render_resource::Extent3d {
                        width: square,
                        height: square,
                        depth_or_array_layers: depth,
                    },
                    bevy::render::render_resource::TextureDimension::D2,
                    data,
                    #format,
                    bevy::render::render_asset::RenderAssetUsages::default(),  // TODO: if there are no CPU image derived features, set to render only
                );
                let #name = images.add(#name);
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
            images: &mut bevy::asset::Assets<bevy::render::texture::Image>,
            planar: &Self::PlanarType,
        ) -> Self {
            #(#buffers)*

            Self {
                #(#buffer_names),*
            }
        }
    }
}


pub fn generate_get_asset_handles_method(fields_named: &FieldsNamed) -> quote::__private::TokenStream {
    let buffer_names = fields_named.named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote! { self.#name.clone() }
        });

    quote! {
        fn get_asset_handles(&self) -> Vec<bevy::asset::Handle<bevy::render::texture::Image>> {
            vec![
                #(#buffer_names),*
            ]
        }
    }
}


struct TextureFormatAttr(Path);

impl Parse for TextureFormatAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let format: Path = input.parse()?;
        Ok(TextureFormatAttr(format))
    }
}

fn extract_texture_format(attributes: &[Attribute]) -> TokenStream {
    for attr in attributes {
        if attr.path().is_ident("texture_format") {
            if let Ok(parsed) = attr.parse_args::<TextureFormatAttr>() {
                let TextureFormatAttr(format) = parsed;
                return quote! { #format };
            } else {
                panic!("error parsing texture_format attribute");
            }
        }
    }

    panic!("no texture_format attribute found, add `#[texture_format(Ident)]` to the field declarations");
}
