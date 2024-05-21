use std::{f32::consts, sync::Arc};
use camera::{Camera, CameraController};
use image::GenericImageView;
use instance::{Instance, InstanceRaw};
use texture::Texture;
use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, Event, KeyEvent, StartCause, WindowEvent}, 
    event_loop::{EventLoop, EventLoopWindowTarget}, 
    keyboard::{Key, NamedKey}, 
    window::{Window, WindowBuilder}
};

mod texture;
mod camera;
mod instance;

#[repr(C)]
#[derive(Debug,Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    // color: [f32; 3],
    tex_coords: [f32; 2],
}

// unsafe impl bytemuck::Pod for Vertex{}
// unsafe impl bytemuck::Zeroable for Vertex{}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0=>Float32x3, 1=>Float32x2];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // 定义一个顶点所占的字节数
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // 确定缓存数组中元素是一个顶点
            step_mode: wgpu::VertexStepMode::Vertex,
            // 描述顶点的布局
            // attributes: &[
            //     wgpu::VertexAttribute {
                //  定义属性的字节偏移
            //         offset: 0,
                // 定义着色器要在什么位置存储这个属性，@location(0) x vec3f
            //         shader_location: 0,
                // 定义该属性的数据格式 Float32x3 => vec3ff
            //         format: wgpu::VertexFormat::Float32x3,
            //     },
            //     wgpu::VertexAttribute {
            //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            //         shader_location: 1,
            //         format: wgpu::VertexFormat::Float32x3,
            //     }
            // ]
            attributes: &Self::ATTRIBS
        }
    }
}

// const VERTICES: &[Vertex] = &[
//     Vertex { position: [0.0,0.5,0.0],color: [1.0,0.0,0.0] },
//     Vertex { position: [-0.5,-0.5,0.0],color: [0.0,1.0,0.0] },
//     Vertex { position: [0.5,-0.5,0.0],color: [0.0,0.0,1.0] },
// ];

// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
//     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E

//     Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
//     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E

//     Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
// ];

// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
// ];

// Changed
// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
// ];

const VERTICES: &[Vertex] = &[
    // 修改后的
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

#[repr(C)]
#[derive(Debug,Clone, Copy,bytemuck::Pod,bytemuck::Zeroable)]
// 视图投影矩阵
struct CameraUniform {
    view_proj: [[f32;4];4]
}



impl CameraUniform {
    fn new()->Self {
        Self { view_proj: glam::Mat4::IDENTITY.to_cols_array_2d() }
    }

    fn update_view_proj(&mut self,camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: Texture,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
}

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: glam::Vec3 = glam::Vec3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5,0.0,NUM_INSTANCES_PER_ROW as f32 * 0.5);

impl State {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // 获取GPU适配器（指向WebGPU API实现的实例）
        // Backends::all() : Vulkan, Metal, DX12, WebGL等后端
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        /*
         * 关于RequestAdapterOptions:
         * power_preference: LowPower(高续航:集成显卡) 和 HighPerformance(高功耗:独立显卡)
         * compatible_surface: 查找兼容surface的适配器
         * force_fallback_adapter: 强制选择一个能在所有系统上工作的适配器，通常意味着使用软渲染
         *
         */
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();
        
        let (device,queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                // 允许我们指定想要的扩展功能，但需要设备支持
                required_features: wgpu::Features::empty(),
                // 该字段用于某些资源限制
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            // 追踪API调用路径 
            None
        ).await.unwrap();

        

        let caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // surface 如何在GPU上存储
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            // 展示平面和显示设备的同步
            // Fifo 指定了显示设备的刷新率做为渲染的帧速率，这本质上就是垂直同步
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            // 延迟帧数？？？
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        // 加载图像
        let diffuse_bytes = include_bytes!("../happy-tree.png");
        // let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        // let diffuse_rgba = diffuse_image.to_rgba8();
        // let dimensions = diffuse_image.dimensions(); 
        let diffuse_texture = Texture::from_bytes(&device,&queue,diffuse_bytes,"happy-tree").unwrap();

        // 创建纹理
        // let texture_size = wgpu::Extent3d {
        //     width: dimensions.0,
        //     height: dimensions.1,
        //     depth_or_array_layers: 1,
        // };
        // let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor{
        //     size:texture_size,
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     // 大多数图像都是使用 sRGB 来存储的，我们需要在这里指定
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     // TEXTURE_BINDING 表示我们要在着色器中使用这个纹理。
        //     // COPY_DST 表示我们能将数据复制到这个纹理上。
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //     label: Some("diffuse_texture"),
        //     view_formats: &[],
        // });

        // 使用命令队列来填充纹理数据
        // queue.write_texture(
        //     // 告诉 wgpu 从何处复制像素数据
        //     wgpu::ImageCopyTexture{
        //     texture: &diffuse_texture,
        //     mip_level: 0,
        //     origin: wgpu::Origin3d::ZERO,
        //     aspect: wgpu::TextureAspect::All,
        // }, &diffuse_rgba, 
        //  // 纹理的内存布局
        // wgpu::ImageDataLayout{
        //     offset:0,
        //     bytes_per_row: Some(4 * dimensions.0),
        //     rows_per_image: Some(dimensions.1),
        // }, texture_size);

        // 创建纹理视图和采样器
        // let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        // let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor{
        //     address_mode_u: wgpu::AddressMode::ClampToEdge, // 边缘拉伸
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Linear, // 线性插值
        //     min_filter: wgpu::FilterMode::Nearest, // 近处会有像素感
        //     mipmap_filter: wgpu::FilterMode::Nearest,
        //     ..Default::default()
        // });

        // 创建绑定组
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[
                // 绑定到 0 资源槽的纹理
                wgpu::BindGroupLayoutEntry {
                    binding:0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // 绑定到 1 资源槽的采样器
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("texture_bind_group")
        });

         // 定义摄像机
         let camera = Camera::new(config.width as f32 / config.height as f32);

         let mut camera_uniform = CameraUniform::new();
         camera_uniform.update_view_proj(&camera);
         let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
             label: Some("camera uniform"),
             contents: bytemuck::cast_slice(&[camera_uniform]),
             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
         });
         let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
             label: Some("camera_bind_group_layout"), 
             entries: &[
                 wgpu::BindGroupLayoutEntry {
                     binding: 0,
                     visibility: wgpu::ShaderStages::VERTEX,
                     ty: wgpu::BindingType::Buffer {
                         ty: wgpu::BufferBindingType::Uniform,
                         has_dynamic_offset: false,
                         min_binding_size: None
                     },
                     count: None
                 }
             ]
         });
         let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
             label:Some("camera_bind_group"),
             layout: &camera_bind_group_layout,
             entries: &[
                 wgpu::BindGroupEntry{
                     binding:0,
                     resource: camera_buffer.as_entire_binding()
                 }
             ]
         });

         // 创建实例缓冲区  10行10列
         let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z|{
            (0..NUM_INSTANCES_PER_ROW).map(move |x|{
                let position = glam::Vec3{x: x as f32, y: 0.0, z: z as f32} - INSTANCE_DISPLACEMENT;
                let rotation = if position.length().abs() <= std::f32::EPSILON {
                    glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                } else {
                    glam::Quat::from_axis_angle(position.normalize(), consts::FRAC_PI_4)
                };

                Instance { position, rotation }
            }) 
         }).collect::<Vec<_>>();
         let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
         let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("instance_buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX
         });

         // 深度纹理
         let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let clear_color = wgpu::Color::BLACK;

        // 着色器
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[],
        });
        // 渲染管线
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                compilation_options: Default::default(),
                entry_point: "vs_main", // 指定函数的入口点
                buffers: &[Vertex::desc(),InstanceRaw::desc()], // 定义传入什么类型的数据到顶点着色器
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                compilation_options: Default::default(),
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState{
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // 混合模式新像素替换旧像素
                    write_mask: wgpu::ColorWrites::ALL, // 允许写入所有颜色通道
                })],
            }),
            // 图元解释如何将顶点数据组织成三角形
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 每3个顶点组成一个三角形
                strip_index_format: None,
                // 确定三角形的朝向（上 左下 右下）
                front_face: wgpu::FrontFace::Ccw, // Ccw指定顶点的帧缓冲区坐标（framebuffer coordinates）按逆时针顺序给出的三角形为朝前（面向屏幕外）
                // 如何剔除三角形
                cull_mode: Some(wgpu::Face::Back), // Back指定朝后（面向屏幕内）的三角形会被剔除（不被渲染）
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            // 启用深度测试
            depth_stencil: Some(wgpu::DepthStencilState{
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                // 使用 LESS 意味着像素将被从后往前绘制，大于当前位置的深度值的像素将被丢弃
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1, // 多采样
                mask: !0, 
                alpha_to_coverage_enabled: false, // 抗锯齿
            },
            multiview: None
        });

        // 顶点缓存区数据
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            // bytemuck::cast_slice() 将数据转换未&[u8]
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = VERTICES.len() as u32;

        // 索引缓存区数据
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let camera_controller = CameraController::new(0.2);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            instances,
            instance_buffer,
            depth_texture
        }
    }

   
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
             // 需要在每次窗口改变时重新配置surface
            self.surface.configure(&self.device, &self.config);
            // 确保更新了 config 之后一定要更新 depth_texture，否则程序就会崩溃，
            // 因为此时 depth_texture 与surface 纹理的宽高已经不一致了
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth texture");
        }
    }

    // 表示一个事件是在此处处理（），如果处理了主循环就不再处理了
    fn input(&mut self, event: &WindowEvent) -> bool {
        // match event {
        //     WindowEvent::KeyboardInput {
        //         event: KeyEvent {
        //             state,
        //             logical_key: Key::Named(NamedKey::Space),
        //             ..
        //         }, .. 
        //     } => {
        //         self.clear_color = if *state == ElementState::Released {
        //             wgpu::Color::BLUE
        //         } else {
        //             wgpu::Color::WHITE
        //         };

        //         true
        //     }

        //     _ => false
        // }
        self.camera_controller.process_events(event)
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 等待surface提供一个SurfaceTexture
        let out = self.surface.get_current_texture()?;
        // 创建一个默认的纹理视图，渲染代码使用纹理视图和纹理进行交互
        let view = out.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // 创建一个命令编码器记录实际命令发送给GPU,(命令编码器会创建一个命令缓冲区)
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });
        {
            // 创建渲染通道来编码所有实际绘制的命令
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),
                // 绑定深度纹理
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment{
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations{
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                color_attachments: &[
                    // 这个时片元着色器中@location(0) 标记指向的颜色附件
                    Some(wgpu::RenderPassColorAttachment{
                    // 要渲染的纹理视图
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear( wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store
                    }
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //告诉 wgpu 用 3 个顶点和 1 个实例（实例的索引就是 @builtin(vertex_index) 的由来）来进行绘制。
            // render_pass.draw(0..self.num_vertices,0..1);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        out.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let mut state = State::new(window.clone()).await;
    // 事件循环处理
    let _ = EventLoop::run(event_loop, 
      move |event: Event<()>,elwt:&EventLoopWindowTarget<()>| {
         if event == Event::NewEvents(StartCause::Init) {
            // 事件启动阶段
         }

         if let Event::WindowEvent {event, ..} = event {
            if !state.input(&event) {
                match event {
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),  
                            ..
                        },
                        ..
                    } | WindowEvent::CloseRequested => elwt.exit(),

                    WindowEvent::Resized(size) => {
                        state.resize(size);
                        window.request_redraw();
                    }

                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {},
                            // 展示平面丢失上下文，需要重新配置
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(e) => eprintln!("{:?}", e),
                        }
                         // 除非我们手动请求，RedrawRequested 将只会触发一次。
                        window.request_redraw();
                    }
                    _ => {}
                }
            }
            
         }   
      });

}