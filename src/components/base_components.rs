//! This module defines base components provided for the user
//! It includes labels, images and buttons.
//! There is also a trait provided that will allow users to define custom components,
//! so that the crate remains as modular and user extendable as possible.

use wgpu::util::DeviceExt;
use winit::window::Window;


use crate::{rendering::{Renderer, Transform}};

use std::{any::Any, usize};

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
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Similar to the `GUIComponent`, except every event gets passed to the component. Useful for buttons
/// and other event driven components.
pub trait EventGUIComponent{
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>) where 'a: 'b;
    fn handle_event_callback(&mut self, event: &winit::event::Event<()>, window: &winit::window::Window);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


/// Similar to a GUI component, but renders text rather than an image.
/// Exists because labels require it.
pub trait TextGUIComponent{
    fn render_text<'a, 'b>(&'a self, brush: &'b mut wgpu_glyph::GlyphBrush<()>) where 'a: 'b;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

    alignment: (wgpu_glyph::VerticalAlign, wgpu_glyph::HorizontalAlign),
    enabled: bool,
}

impl Label{
    /// Create a new `Label` struct
    pub fn new<S: Into<String> + Copy>(content: S, size: f32, pos: [f32; 2]) -> Self{
        Self{
            content: content.into(),
            size,
            pos,
            alignment: (wgpu_glyph::VerticalAlign::Top, wgpu_glyph::HorizontalAlign::Left),
            enabled: true,
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

    pub fn enable(&mut self){
        self.enabled = true;
    }

    pub fn disable(&mut self){
        self.enabled = false;
    }
}

impl TextGUIComponent for Label{
    fn render_text<'a, 'b>(&'a self, brush: &'b mut wgpu_glyph::GlyphBrush<()>)
    where 'a: 'b {
        if self.enabled{
            brush.queue(
                wgpu_glyph::Section {
                    screen_position: (self.pos[0], self.pos[1]),
                    text: vec![wgpu_glyph::Text::new(&self.content).with_color([0.0, 0.0, 0.0, 1.0]).with_scale(wgpu_glyph::ab_glyph::PxScale::from(self.size))],
                    layout: wgpu_glyph::Layout::default().v_align(self.alignment.0).h_align(self.alignment.1),
                    ..wgpu_glyph::Section::default()
                }
                
            )
        }
        println!("Label enabled: {:?}", self.enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
pub struct Button{
    transform: Transform, // position scale and rot
    callback: Option<Box<dyn Fn(&winit::event::Event<()>, &Window, &bool, &mut bool) -> ()>>, // func to run when clicked
    cursor_in_bounds: bool, // tells us if the cursor is in bounds of the button
    vertex_buffer: wgpu::Buffer, // the vertex buffer that stores the verticies of,
    enabled: bool,
    attached_text_id: Option<usize>,
}



impl Button{
    pub fn new(transform: Transform, callback: Option<Box<dyn Fn(&winit::event::Event<()>, &Window, &bool, &mut bool) -> ()>>, renderer: &Renderer, mut text: Option<&mut Label>, attached_text_id: Option<usize>) -> Self{
        if text.is_some(){
            text.take().unwrap().pos = [(transform.position.x + (renderer.sc_desc.width/2) as f32), (transform.position.y + (renderer.sc_desc.height/2) as f32)];
        }
        Self{
            transform,
            callback,
            cursor_in_bounds: false,
            vertex_buffer: create_buffers(&renderer.device),
            enabled: true,
            attached_text_id
        }
    }

    pub fn enable(&mut self){
        self.enabled = true;
    }
    pub fn disable(&mut self){
        self.enabled = false;
    }

    pub fn set_text(&mut self, mut text: Option<&'static mut Label>, renderer: &Renderer){
        if text.is_some(){
            text.take().unwrap().pos = [(self.transform.position.x + (renderer.sc_desc.width/2) as f32), (self.transform.position.y + (renderer.sc_desc.height/2) as f32)];
        }
    }
}


impl EventGUIComponent for Button{
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>)
    where 'a: 'b {
        if self.enabled{
            render_pass.set_bind_group(1, &self.transform.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
    }

    fn handle_event_callback(&mut self, event: &winit::event::Event<()>, window: &winit::window::Window){
        match event{
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if (&window.id() == window_id) => {
                match event{
                    winit::event::WindowEvent::CursorMoved{mut position, ..} => {
                        // Convert window space into WGPU (dx) space
                        position.x -= (window.inner_size().width/2) as f64;
                        position.y -= (window.inner_size().height/2) as f64;

                        // Simple and fast check for collision with mouse - I don't know how I got these values,
                        // I was trying anything to see what stuck
                        if     (((self.transform.position.x - ((self.transform.scale.x*2.0) * (window.inner_size().width/2) as f32) / 2.0) as f64) < position.x 
                            && ((self.transform.position.y - ((self.transform.scale.y*2.0) * (window.inner_size().height/2) as f32) / 2.0) as f64) < position.y) 
                            && (((self.transform.position.x + ((self.transform.scale.x*2.0) * (window.inner_size().width/2) as f32) / 2.0) as f64) > position.x 
                            && ((self.transform.position.y + ((self.transform.scale.y*2.0) * (window.inner_size().height/2) as f32) / 2.0) as f64) > position.y){
                            self.cursor_in_bounds = true;
                        }else{
                            self.cursor_in_bounds = false;
                        }
                    }
                
                    _ => {}
            }
        }

            _ => {}
        }
        // We now callback the user callback
        match &self.callback{
            Some(v) => { v(event, &window, &self.cursor_in_bounds, &mut self.enabled);},
            None => {}
        };

        // If we have some text, then enable and disable according to our button (as text shouldn't be enabled if the button isn't)
        /*if self.text.is_some(){
            match self.enabled{
                true => self.text.take().unwrap().enable(),
                false => self.text.take().unwrap().disable(),
            };
            println!("Button enabled: -> {:?}", self.enabled);
        }*/
       
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

