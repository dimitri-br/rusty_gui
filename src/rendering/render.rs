//! This module contains everything needed by our GUI app to render using wgpu-rs.
//! It will take a component list (Which is the GUI objects we want to render),
//! and then get the required resources - like font assets, images and more,
//! and render it to the screen as a quad. This module does little more than,
//! render!





use wgpu::{BindGroup, Device, MultisampleState, PrimitiveState, util::StagingBelt};

use crate::{components::{Label}, layout::{Layout}};

use super::TransformUniform;

/// # Renderer
///
/// The renderer struct holds all the data we need to render, and
/// provides a higher level abstraction over wgpu-rs to render our GUI
pub struct Renderer{
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
    staging_belt: StagingBelt,

    glyph_brush: wgpu_glyph::GlyphBrush<()>,

    pub layout: Layout,    

    camera: Camera,
}


impl Renderer{
    /// Create a new renderer, initializing all values
    pub async fn new(window: &winit::window::Window) -> Self{
        // Set our size to the window size
        let size = window.inner_size();


        // Create a new instance with the best api (VULKAN, DX12 or METAL)
        let instance = wgpu::Instance::new(wgpu::BackendBit::DX11);

        // Create a surface (like a link to the winit window)
        
            let surface = unsafe { instance.create_surface(window) };

        // Create our adapter. We can select things like the power preference
        // and define the surface to draw to.
        // We want low power as we're not drawing games and the like.
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        // Request the device and queue. This can be thought of as a link to the GPU,
        // and the queue is like a pipe to render down (eg, compute or graphics).
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: Some("Device Descriptor"),
            },
            None, // Trace path
        ).await.unwrap();

        // We define what a swapchain should be - eg, its usage, format (RGB, BGR)
        // size, width and present mode - vsync on or off for example.
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        // create a swapchain using the swapchain description and link it to the surface
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let render_pipeline = Renderer::create_render_pipeline(&device);

        let staging_belt = StagingBelt::new(2048);

        
        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!("../../fonts/FingerPaint-Regular.ttf"))
        .expect("Load font");

        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
            .build(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        let layout = Layout::new();

        let camera = Camera::new(0.1, 750.0, &device, &sc_desc);

        Self{
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            render_pipeline,
            staging_belt,
            glyph_brush,
            layout,
            camera
        }
    }

    /// Create a render pipeline from default values, taking in a reference to the device
    pub fn create_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline{
        // Define our pipeline layout. This is where we define bind_group_layouts
        let render_pipeline_layout =
       device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
           label: Some("Render Pipeline Layout"),
           bind_group_layouts: &[
               &TransformUniform::create_bind_group_layout(device),
               &TransformUniform::create_bind_group_layout(device)
           ],
           push_constant_ranges: &[],
        });

        // Create our shader modules
        let vs_module = device.create_shader_module(&wgpu::include_spirv!("../../shaders/shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("../../shaders/shader.frag.spv"));

        // Create the pipeline. We define it - we're rendering a GUI, so it doesn't matter much
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main", // 1.
                buffers: &[Vertex::desc()]
            },
            fragment: Some(wgpu::FragmentState { // 2.
                module: &fs_module,
                entry_point: "main",
                targets: &[
                    wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        color_blend: wgpu::BlendState {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },
                        alpha_blend: wgpu::BlendState {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },
                        //color_blend: wgpu::BlendDescriptor::REPLACE,
                        //alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL
                    }
                ],
            }),

            primitive: PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(wgpu::IndexFormat::Uint32),

                front_face: wgpu::FrontFace::Ccw,

                cull_mode: wgpu::CullMode::Back,

                polygon_mode: wgpu::PolygonMode::Fill,
            },

            depth_stencil: None,

            multisample: MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: true
            },
        })
    }

    /// This function gets called upon a resize, as we need to recreate the swapchain
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        if new_size.width > 0 && new_size.height > 0{
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        }
    }

    /// This should run BEFORE we render. This lets us set up last minute values
    /// and update our layout before we render
    pub fn prepass(&mut self){
        let mut text_child_components = Vec::<(usize, bool, [f32; 2])>::new();
        let components = &self.layout.components;
        for i in 0..components.len(){
            let comp = &components[i];
            if let Some(id) = comp.get_text_id(){
                text_child_components.push((id, comp.is_enabled(), comp.get_pos()));
            }
        }
        let components = &self.layout.event_components;
        for i in 0..components.len() {
            let comp = &components[i];
            if let Some(id) = comp.get_text_id(){
                text_child_components.push((id, comp.is_enabled(), comp.get_pos()));
            }
        }

        for (id, enabled, pos) in text_child_components.iter(){
            let text = self.layout.borrow_text_component_as_type_mut::<Label>(*id).unwrap();
            text.set_pos(*pos, (self.sc_desc.width, self.sc_desc.height));
            if *enabled{
                text.enable();
            }else{
                text.disable();
            }
        }
    }

    /// Render a single frame 
    pub fn render(&mut self, clear_color: wgpu::Color){
        let frame = self.swap_chain.get_current_frame().unwrap().output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });   

        self.camera.update(&self.sc_desc);

        {
            // Pre pass
            // Main pass - Render all our shaders and objects to the screen
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        }
                    },
                ],
                depth_stencil_attachment: None,
                label: Some("render pass descriptor"),
            });

            render_pass.set_pipeline(&self.render_pipeline);



            {   
                let components = &self.layout.components;
                for i in 0..components.len(){
                    let comp = &components[i];
                    render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
                    comp.render(&mut render_pass);
                }
            }
            {
                let components = &self.layout.event_components;
                for i in 0..components.len() {
                    let comp = &components[i];
                    render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
                    comp.render(&mut render_pass);
                }
            }
            {
                for text_comp in self.layout.text_components.iter(){
                    text_comp.render_text(&mut self.glyph_brush);
                }
            }
        }

        {
            self.glyph_brush.draw_queued(&self.device, &mut self.staging_belt, &mut encoder, &frame.view, self.sc_desc.width, self.sc_desc.height).unwrap();
        }

        self.staging_belt.finish();
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}


/// # Vertex
/// 
/// This struct defines how a vertex should look in a shader. We define a 
/// position and tex_coord, as we only want to render 2d images (sprites)
/// for the GUI. We define it as being a C like struct so we have even spacing
/// which is required for GLSL
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2], // NEW!
}

impl Vertex {
    /// Create a description of how this struct should look in a shader
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2, // NEW!
                },
            ]
        }
    }
}

/// This is a helpful quad type to help you render sprites to the screen
pub const QUAD: &[Vertex] = &[
    // Changed
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], }, // A
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
    
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // A
    
]; 


use cgmath::{Matrix4, SquareMatrix};
use wgpu::util::DeviceExt;
#[derive(Debug)]
pub struct Camera {
    pub near: f32,
    pub far: f32,

    pub width: u32,
    pub height: u32,

    camera_uniform: CameraUniform,
    buffer: wgpu::Buffer,

    bind_group: BindGroup,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


impl Camera {
    pub fn new(near: f32, far: f32, device: &Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self{
        let mut camera_uniform = CameraUniform::new();
        let proj = cgmath::ortho(0.0, sc_desc.width as f32, sc_desc.height as f32, 0.0, 0.0, 1000.0);
        camera_uniform.update_view_proj(proj);
        let buffer = camera_uniform.create_uniform_buffer(device);
        let bind_group = CameraUniform::create_bind_group(device, &buffer);
        Self{
            near,
            far,
            width: 0,
            height: 0,
            camera_uniform,
            buffer,
            bind_group,
        }
    }
    pub fn build_view_projection_matrix(&mut self, sc_desc: &wgpu::SwapChainDescriptor) -> cgmath::Matrix4<f32>{
        self.width = sc_desc.width;
        self.height = sc_desc.height;
        // 1.
        // 2.
        let proj = cgmath::ortho(0.0, self.width as f32, self.height as f32, 0.0, 0.0, 1000.0);

        let view = cgmath::Matrix4::<f32>::look_at_rh(
            cgmath::Point3::<f32>::new(0.0, 0.0, 5.0), 
            cgmath::Point3::<f32>::new(0.0, 0.0, 0.0), 
            cgmath::Vector3::<f32>::new(0.0, 1.0, 0.0)
        );
        
        // 3.
        return OPENGL_TO_WGPU_MATRIX * (proj * view);
    }

    pub fn update(&mut self, sc_desc: &wgpu::SwapChainDescriptor){
        let value = self.build_view_projection_matrix(sc_desc);
        self.camera_uniform.update_view_proj(value);
    }
}

 


// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform{
    pub proj: [[f32; 4]; 4],
}

impl CameraUniform{
    pub fn new() -> Self {
        Self {
            proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, proj: Matrix4<f32>) {
        self.proj = proj.into();
    }

    pub fn create_uniform_buffer(&self, device: &Device) -> wgpu::Buffer{
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }

    pub fn create_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout{
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: None,
                        ty: wgpu::BufferBindingType::Uniform
                    },
                    count: None,
                }
            ],
            label: Some("Transform_Bind_Layout"),
        })
    }

    pub fn create_bind_group(device: &Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup{
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &CameraUniform::create_bind_group_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ],
            label: Some("Transform_Bind_Group"),
        })
    }
}