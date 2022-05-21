use pollster;
use winit::{
    event::*,
    event_loop::{ ControlFlow, EventLoop },
    window::{ WindowBuilder, Window }
};
use wgpu;

#[macro_use]
extern crate log;



struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,

    clear_color: wgpu::Color
}

impl State {

    async fn new(window: &Window) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                label: None
            },
            None
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo
        };
        surface.configure(&device, &config);


        let clear_color = wgpu::Color::GREEN;


        Self {
            surface, device, queue, config, window_size,
            clear_color
        }
    }



    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: true
                        }
                    }
                ],
                depth_stencil_attachment: None
            });

        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }



    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.window_size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

}



async fn run(event_loop: EventLoop<()>, window: Window) {

    let mut state = pollster::block_on(State::new(&window));


    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            match state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => state.resize(state.window_size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => error!("{:?}", e)
            }
        },
        Event::RedrawEventsCleared => {
            window.request_redraw();
        },
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => {}
    });

}



fn _main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    pollster::block_on(run(event_loop, window));

}



#[cfg(target_os="android")]
#[no_mangle]
extern "C" fn android_main() {
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Trace)
    );

    _main();
}

#[cfg(target_os="android")]
fn main() {}

#[cfg(not(target_os="android"))]
fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    _main();
}