//! This module contains everything needed by our GUI app to render using wgpu-rs.
//! It will take a component list (Which is the GUI objects we want to render),
//! and then get the required resources - like font assets, images and more,
//! and render it to the screen as a quad. This module does little more than,
//! render!

use crate::components::{Label, GUIComponent};

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
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
}


impl Renderer{
    /// Create a new renderer, initializing all values
    pub async fn new(window: &winit::window::Window) -> Self{
        // Set our size to the window size
        let size = window.inner_size();


        // Create a new instance with the best api (VULKAN, DX12 or METAL)
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

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
                shader_validation: true,
            },
            None, // Trace path
        ).await.unwrap();

        // We define what a swapchain should be - eg, its usage, format (RGB, BGR)
        // size, width and present mode - vsync on or off for example.
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        // create a swapchain using the swapchain description and link it to the surface
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let render_pipeline = Renderer::create_render_pipeline(&device);

        Self{
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            render_pipeline,
        }
    }

    /// Create a render pipeline from default values, taking in a reference to the device
    pub fn create_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline{
        // Define our pipeline layout. This is where we define bind_group_layouts
        let render_pipeline_layout =
       device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
           label: Some("Render Pipeline Layout"),
           bind_group_layouts: &[],
           push_constant_ranges: &[],
        });

        // Create our shader modules
        let vs_module = device.create_shader_module(wgpu::include_spirv!("../../shaders/shader.vert.spv"));
        let fs_module = device.create_shader_module(wgpu::include_spirv!("../../shaders/shader.frag.spv"));

        // Create the pipeline. We define it - we're rendering a GUI, so it doesn't matter much
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main", // 1.
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(
                wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                    clamp_depth: false,
                }
            ),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add
                    },
                    //color_blend: wgpu::BlendDescriptor::REPLACE,
                    //alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL
                }
            ],

            primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.

            depth_stencil_state: None,

            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1, // 5.
            sample_mask: !0, // 6.
            alpha_to_coverage_enabled: true, // 7.
        })
    }

    /// This function gets called upon a resize, as we need to recreate the swapchain
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    /// Render a single frame 
    pub fn render(&mut self){
        let frame = self.swap_chain.get_current_frame().unwrap().output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });   

        let mut label = Label::new("hellO", 64, [64, 64], &self.device);

        {
            // Pre pass
            // Main pass - Render all our shaders and objects to the screen
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: true,
                        }
                    },
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);


            label.render(&mut render_pass);
        }

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
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2, // NEW!
                },
            ]
        }
    }
}

/// This is a helpful quad type to help you render sprites to the screen
pub const Quad: &[Vertex] = &[
    // Changed
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], }, // A
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
    
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // A
    
]; 