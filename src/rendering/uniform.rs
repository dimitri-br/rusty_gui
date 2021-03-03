//! This module helps make buffer and bind group/layouts easy to initialize.

use std::num::NonZeroU64;
use wgpu::util::DeviceExt;

/// UniformUtils is a tool meant to simplify uniform buffer creation
pub struct UniformUtils;

impl UniformUtils{
    /// All in one creation tool with some default values. Returns the bind group, layout and buffer.
    pub fn create<T: bytemuck::Pod>(device: &wgpu::Device, visibility: wgpu::ShaderStage, binding: u32, uniform: &T, label: &str) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout){
        let layout = UniformUtils::create_bind_group_layout(device, binding, visibility, false, None, label);
        let buffer = UniformUtils::create_uniform_buffer(device, uniform);
        let bind_group = UniformUtils::create_bind_group(device, &layout, binding, &buffer, label);

        (buffer, bind_group, layout)
    }

    /// Create a buffer from uniform that derives from `Pod`
    pub fn create_uniform_buffer<T: bytemuck::Pod>(device: &wgpu::Device, uniform: &T) -> wgpu::Buffer{
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*uniform]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }

    /// Create a new bind group layout based on various parameters
    pub fn create_bind_group_layout(device: &wgpu::Device, binding: u32, visibility: wgpu::ShaderStage, dynamic: bool, min_binding_size: Option<NonZeroU64>, label: &str) -> wgpu::BindGroupLayout{
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding,
                    visibility,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic,
                        min_binding_size,
                    },
                    count: None,
                }
            ],
            label: Some(label),
        })
    }

    /// Create a new bind group, based off a layout and binding with the buffer.
    pub fn create_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, binding: u32, buffer: &wgpu::Buffer, label: &str) -> wgpu::BindGroup{
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding,
                    resource: wgpu::BindingResource::Buffer(buffer.slice(..))
                }
            ],
            label: Some(label),
        })
    }
}