//! This module defines base components provided for the user
//! It includes labels, images and buttons.
//! There is also a trait provided that will allow users to define custom components,
//! so that the crate remains as modular and user extendable as possible.

use wgpu::util::DeviceExt;


use crate::{rendering::Transform};



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

/// Similar to the `GUIComponent`, except every event gets passed to the component. Useful for buttons
/// and other event driven components.
pub trait EventGUIComponent{
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b;
    fn handle_event_callback(&self, event: &winit::event::Event<()>, window: &winit::window::Window);
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
pub struct Label{
    content: String,
    size: f32,
    pos: [f32; 2], // x and y coords

    alignment: (wgpu_glyph::VerticalAlign, wgpu_glyph::HorizontalAlign)
}

impl Label{
    /// Create a new `Label` struct
    pub fn new<S: Into<String> + Copy>(content: S, size: f32, pos: [f32; 2]) -> Self{
        Self{
            content: content.into(),
            size,
            pos,
            alignment: (wgpu_glyph::VerticalAlign::Top, wgpu_glyph::HorizontalAlign::Left)
        }
    }

    /// Change the vertical alignment of the label
    pub fn align_vertical(&mut self, alignment: wgpu_glyph::VerticalAlign){
        self.alignment.0 = alignment;
    }

    /// Change the horizontal alignment of the label
    pub fn align_horizontal(&mut self, alignment: wgpu_glyph::HorizontalAlign){
        self.alignment.1 = alignment;
    }
}

impl TextGUIComponent for Label{
    fn render_text<'a, 'b>(&'a self, brush: &'b mut wgpu_glyph::GlyphBrush<()>)
    where 'a: 'b {
        brush.queue(
            wgpu_glyph::Section {
                screen_position: (self.pos[0], self.pos[1]),
                text: vec![wgpu_glyph::Text::new(&self.content).with_color([0.0, 0.0, 0.0, 1.0]).with_scale(wgpu_glyph::ab_glyph::PxScale::from(self.size))],
                layout: wgpu_glyph::Layout::default().v_align(self.alignment.0).h_align(self.alignment.1),
                ..wgpu_glyph::Section::default()
            }
            
        )
    }
}


/// # Button
///
/// A button component. It implements a label struct as a child.
/// All buttons run through the event handler (not the user defined one),
/// so inputs are registered.
///
/// The button also contains the callback to run when the button is pressed.
///
/// This is designed to be a simple, no frills button. If you want to implement animated buttons,
/// feel free to make your own components
pub struct Button<F> where F: Fn() -> (){
    transform: Transform, // position scale and rot
    callback: Box<F>, // func to run when clicked
}


impl<F> Button<F> where F: Fn() -> (){
    pub fn new(transform: Transform, callback: Box<F>) -> Self{
        Self{
            transform,
            callback,
        }
    }


}


impl<F> EventGUIComponent for Button<F> where F: Fn() -> (){
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>)
    where 'a: 'b {
        // TODO
    }

    fn handle_event_callback(&self, event: &winit::event::Event<()>, window: &winit::window::Window){
        match event{
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if (&window.id() == window_id) => {
                match event{
                    winit::event::WindowEvent::CursorMoved{position, ..} => {
                        println!("Top left: ({:?}, {:?})", (self.transform.position.x - (self.transform.scale.x / 2.0)), (self.transform.position.y - (self.transform.scale.y / 2.0)));
                        println!("Bottom right: ({:?}, {:?})", (self.transform.position.x + (self.transform.scale.x / 2.0)), (self.transform.position.y + (self.transform.scale.y / 2.0)));
                        
                        println!("Cursor pos: ({:?}, {:?}", position.x, position.y);
                        
                        println!("Bounds: {:?}", (((self.transform.position.x - self.transform.scale.x / 2.0) as f64) < position.x && ((self.transform.position.y - self.transform.scale.y / 2.0) as f64) < position.y) && (((self.transform.position.x + self.transform.scale.x / 2.0) as f64) > position.x && ((self.transform.position.y + self.transform.scale.y / 2.0) as f64) > position.y));
                        if (((self.transform.position.x - self.transform.scale.x / 2.0) as f64) < position.x && ((self.transform.position.y - self.transform.scale.y / 2.0) as f64) < position.y) && (((self.transform.position.x + self.transform.scale.x / 2.0) as f64) > position.x && ((self.transform.position.y + self.transform.scale.y / 2.0) as f64) > position.y){
                            println!("Positon {:?} in bounds!", position);
                        }
                    }
                
                    _ => {}
            }
        }

            _ => {}
        }
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