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
/// Lastly, the user should define a new function to easily create a new struct.
pub trait GUIComponent{
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b;
}

/// Similar to a GUI component, but renders text rather than an image.
/// Exists because labels require it.
pub trait TextGUIComponent{
    fn render_text<'a, 'b>(&'a self, brush: &'b mut wgpu_glyph::GlyphBrush<()>) where 'a: 'b;
}

// This part now shows some of the base components, and may help when designing your own custom components


/// # Label
///
/// This works like many labels in GUI libraries - renders
/// text to the screen, using a specified size, pos and font.
pub struct Label<S: Into<String> + Copy>{
    content: S,
    size: f32,
    pos: [f32; 2], // x and y coords
}

impl<S: Into<String> + Copy> Label<S>{
    /// Create a new `Label` struct
    pub fn new(content: S, size: f32, pos: [f32; 2]) -> Self{
        Self{
            content,
            size,
            pos,
        }
    }
}

impl<S: Into<String> + Copy> TextGUIComponent for Label<S>{
    fn render_text<'a, 'b>(&'a self, brush: &'b mut wgpu_glyph::GlyphBrush<()>)
    where 'a: 'b {
        let text: String = self.content.into();
        brush.queue(
            wgpu_glyph::Section {
                screen_position: (self.pos[0], self.pos[1]),
                text: vec![wgpu_glyph::Text::new(&text).with_color([0.0, 0.0, 0.0, 1.0]).with_scale(wgpu_glyph::ab_glyph::PxScale::from(self.size))],
                layout: wgpu_glyph::Layout::default(),
                ..wgpu_glyph::Section::default()
            }
            
        )
    }
}

/// Helpful function to automatically create a new quad buffer for all your GUI needs.
pub fn create_buffers(device: &wgpu::Device) -> wgpu::Buffer{
    // Create the vertex buffer (so we can draw to it)
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(crate::rendering::QUAD),
            usage: wgpu::BufferUsage::VERTEX,
        }
    )

}