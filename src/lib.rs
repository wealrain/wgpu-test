use std::{sync::Arc, thread::sleep_ms};
use winit::{
    event::{ElementState, Event, KeyEvent, StartCause, WindowEvent}, 
    event_loop::{EventLoop, EventLoopWindowTarget}, 
    keyboard::{Key, NamedKey}, 
    window::{Window, WindowBuilder}
};

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
}

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

        let clear_color = wgpu::Color::BLACK;

        // 着色器
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                compilation_options: Default::default(),
                entry_point: "vs_main", // 指定函数的入口点
                buffers: &[], // 定义传入什么类型的数据到顶点着色器
    
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
                // 确定三角形的朝向
                front_face: wgpu::FrontFace::Ccw, // Ccw指定顶点的帧缓冲区坐标（framebuffer coordinates）按逆时针顺序给出的三角形为朝前（面向屏幕外）
                // 如何剔除三角形
                cull_mode: Some(wgpu::Face::Back), // Back指定朝后（面向屏幕内）的三角形会被剔除（不被渲染）
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
    
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // 多采样
                mask: !0, 
                alpha_to_coverage_enabled: false, // 抗锯齿
            },
            multiview: None
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline
        }
    }

   
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
             // 需要在每次窗口改变时重新配置surface
            self.surface.configure(&self.device, &self.config);
        }
    }

    // 表示一个事件是在此处处理（），如果处理了主循环就不再处理了
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    logical_key: Key::Named(NamedKey::Space),
                    ..
                }, .. 
            } => {
                self.clear_color = if *state == ElementState::Released {
                    wgpu::Color::BLUE
                } else {
                    wgpu::Color::WHITE
                };

                true
            }

            _ => false
        }
    }

    fn update(&mut self) {
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
            //告诉 wgpu 用 3 个顶点和 1 个实例（实例的索引就是 @builtin(vertex_index) 的由来）来进行绘制。
            render_pass.draw(0..3,0..1);
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