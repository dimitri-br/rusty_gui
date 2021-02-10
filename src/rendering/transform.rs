//! This module contains the `Transform` struct, which defines a transformation when rendering (and in general)
//! This can be used to translate, scale and rotate GUI components.

use wgpu::Device;
use wgpu::util::DeviceExt;

use cgmath::SquareMatrix;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Transform{
    pub position: cgmath::Vector3::<f32>,
    pub rotation: cgmath::Quaternion::<f32>,
    pub scale: cgmath::Vector3::<f32>,
    value: cgmath::Matrix4::<f32>,
    uniform: TransformUniform,
    buffer: wgpu::Buffer,
}
impl Transform{
    /// Create a new transform. Takes in the position, rotation and scale values.
    pub fn new(position: cgmath::Vector3::<f32>, rotation: cgmath::Quaternion::<f32>, scale: cgmath::Vector3::<f32>, device: &Device) -> Self{
        let value: cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(position) * cgmath::Matrix4::from(rotation) * cgmath::Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        let mut uniform = TransformUniform::new();
        uniform.update(value);


        let buffer = uniform.create_uniform_buffer(device);

        Self{
            position,
            rotation,
            scale,
            value,
            uniform,
            buffer,
        }
    }


    pub fn get_buffer(&mut self, device: &Device) -> &wgpu::Buffer{
        self.value = cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation) * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.uniform.update(self.value);
        self.buffer = self.uniform.create_uniform_buffer(device);
        &self.buffer
    }
}





// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform{
    transform: [[f32; 4]; 4] // Store our rotation as a 4x4 matrix
}
impl TransformUniform{
    pub fn new() -> Self{
        Self{
            transform: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update(&mut self, value: cgmath::Matrix4::<f32>){
        let value = value * OPENGL_TO_WGPU_MATRIX;
        self.transform = value.into();
        
    }

    pub fn create_uniform_buffer(&self, device: &Device) -> wgpu::Buffer{
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Rotation Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }
}