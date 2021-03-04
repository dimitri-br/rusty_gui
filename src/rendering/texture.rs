//! This module controls the various functions to load and store
//! textures for use in the GUI app. It works by storing textures
//! and the various buffers/bind groups in a hashmap as a pool,
//! to avoid reloading textures over and over.

use std::{collections::HashMap, ops::Deref};
use super::{Renderer, UniformUtils, render};
use image::GenericImageView;
use wgpu::BindGroup;


pub struct TexturePool{
    pub pool: HashMap<&'static str, wgpu::BindGroup>
}

impl TexturePool{
    pub fn new() -> Self{
        Self{
            pool: HashMap::<&'static str, wgpu::BindGroup>::new()
        }
    }

    pub fn add_texture(&mut self, name: &'static str, texture: BindGroup){
        self.pool.insert(name, texture);
    }

    pub fn get_texture(&self, name: &'static str) -> Option<&BindGroup>{
        self.pool.get(name)
    }
}

pub struct Texture{
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,

    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout
}

impl Texture{
    pub fn from_path(path: &'static str, renderer: &Renderer) -> Self{

        let loaded_image = image::open(path).expect("image failed to load");
        let rgba = loaded_image.as_rgba8().expect("Image failed to load/convert as RGBA8!");
        let dimensions = loaded_image.dimensions();

        let queue = renderer.queue;
        let device = renderer.device;

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Image"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        todo!()
    }
}

/// This struct holds useful utilities to create textures.
pub struct TextureUtils;