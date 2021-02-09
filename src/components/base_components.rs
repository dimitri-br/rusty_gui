//! This module defines base components provided for the user
//! It includes labels, images and buttons.
//! There is also a trait provided that will allow users to define custom components,
//! so that the crate remains as modular and user extendable as possible.

use wgpu::util::DeviceExt;

/// # GUIComponent
///
/// This trait defines a GUIComponent
/// The component will be rendered as long as it is in a layout
/// that is currently getting drawn by the renderer.
///
/// It has a single function - render, which will define how the component gets
/// renderered onto the screen.
///
/// You use this function to add images to bind groups, or to draw text etc.
///
/// There is also the create_buffers method, which should create a vertex buffer from the quad primitive type
///
/// Lastly, the user should define a new function to easily create a new struct.
pub trait GUIComponent{
    fn create_buffers(device: &wgpu::Device) -> wgpu::Buffer{
        // Create the vertex buffer (so we can draw to it)
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(crate::rendering::Quad),
                usage: wgpu::BufferUsage::VERTEX,
            }
        )

    }

    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b;
}

/// Similar to a GUI component, but renders text rather than an image.
/// Exists because labels require it.
pub trait TextGUIComponent{

}

// This part now shows some of the base components, and may help when designing your own custom components


/// # Label
///
/// This works like many labels in GUI libraries - renders
/// text to the screen, using a specified size, pos and font.
pub struct Label<S: Into<String>>{
    content: S,
    size: u32,
    pos: [u32; 2], // x and y coords

    vertex_buffer: wgpu::Buffer,
}

impl<S: Into<String>> Label<S>{
    /// Create a new `Label` struct, initalizing a new vertex buffer too.
    pub fn new(content: S, size: u32, pos: [u32; 2], device: &wgpu::Device) -> Self{
        Self{
            content,
            size,
            pos,

            vertex_buffer: Self::create_buffers(device)
        }
    }
}

impl<S: Into<String>> GUIComponent for Label<S>{
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}

