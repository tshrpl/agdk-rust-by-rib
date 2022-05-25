


use cgmath;
use pollster;
use winit::{
	event::*,
	event_loop::{ ControlFlow, EventLoop, EventLoopWindowTarget },
	window::{ WindowBuilder, Window }
};
use wgpu;
use wgpu::util::DeviceExt;

use std::{
	ops::Deref,
	sync::{ Arc, RwLock },
	borrow::Cow,
	time::{ Duration, Instant }
};

#[macro_use]
extern crate log;

// mod texture;
mod camera;



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
	view_proj: [[f32; 4]; 4],
	eye: [f32; 3],
	_padding: f32
}

impl CameraUniform {
	fn new() -> Self {
		use cgmath::SquareMatrix;
		Self {
			view_proj: cgmath::Matrix4::identity().into(),
			eye: [0.0; 3],
			_padding: 0.0
		}
	}

	fn update(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
		self.eye = camera.position.into();
		self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
	}
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
	time: f32
}

impl Uniforms {
	fn new() -> Self {
		Self {
			time: 0.0
		}
	}

	fn update(&mut self, time: &Duration) {
		self.time = time.as_secs_f32();
	}
}



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
	position: [f32; 2],
}

impl Vertex {
	fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2
				}
			]
		}
	}
}

const VERTICES: [Vertex; 4] = [
	Vertex { position: [-1.0, -1.0] },
	Vertex { position: [-1.0,  1.0] },
	Vertex { position: [ 1.0,  1.0] },
	Vertex { position: [ 1.0, -1.0] }
];

const INDICES: &[u16] = &[
	2, 1, 0,
	3, 2, 0
];



struct SurfaceState {
	window: Window,
	surface: wgpu::Surface,
}

impl SurfaceState {
	fn new<T>(
		event_loop: &EventLoopWindowTarget<T>,
		instance: &wgpu::Instance
	) -> Self {
		let window = WindowBuilder::new().build(&event_loop).unwrap();

		let surface = unsafe { instance.create_surface(&window) };

		Self { window, surface }
	}

	// fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface_state: &mut SurfaceState) {
	// 	if new_size.width > 0 && new_size.height > 0 {
	// 		self.window_size = new_size;
	// 		self.config.width = new_size.width;
	// 		self.config.height = new_size.height;
	// 		surface_state.surface.configure(&self.device, &self.config);
	// 	}
	// }

}

struct RenderState {
	device: wgpu::Device,
	queue: wgpu::Queue,
	target_format: wgpu::TextureFormat,

	vertex_buffer: wgpu::Buffer,
	num_vertices: u32,
	index_buffer: wgpu::Buffer,
	num_indices: u32,

	render_pipeline: wgpu::RenderPipeline,

	camera_uniform: CameraUniform,
	camera_buffer: wgpu::Buffer,
	camera_bind_group: wgpu::BindGroup,

	uniforms: Uniforms,
	uniform_buffer: wgpu::Buffer,
	uniform_bind_group: wgpu::BindGroup
}

impl RenderState {

	async fn new(
		adapter: &wgpu::Adapter,
		target_format: wgpu::TextureFormat
	) -> Self {
		let (device, queue) = adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
				label: None
			},
			None
		).await.expect("Failed to create device");


		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
		});

		let vertices = VERTICES;

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: bytemuck::cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
			}
		);
		let num_vertices = vertices.len() as u32;

		let index_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: bytemuck::cast_slice(INDICES),
				usage: wgpu::BufferUsages::INDEX
			}
		);
		let num_indices = INDICES.len() as u32;


		let camera_uniform = CameraUniform::new();

		let camera_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor{
				label: Some("Camera Buffer"),
				contents: bytemuck::cast_slice(&[camera_uniform]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}
		);

		let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				}
			],
			label: Some("camera_bind_group_layout")
		});

		let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &camera_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: camera_buffer.as_entire_binding()
				}
			],
			label: Some("camera_bind_group")
		});


		let mut uniforms = Uniforms::new();

		let uniform_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor{
				label: Some("Uniform Buffer"),
				contents: bytemuck::cast_slice(&[uniforms]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
			}	
		);

		let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				}
			],
			label: Some("uniform_bind_group_layout")
		});

		let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &uniform_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: uniform_buffer.as_entire_binding()
				}
			],
			label: Some("uniform_bind_group")
		});


		trace!("WGPU: creating pipeline layout");
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[
				&camera_bind_group_layout,
				&uniform_bind_group_layout
			],
			push_constant_ranges: &[],
		});

		trace!("WGPU: creating render pipeline");
		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[
					Vertex::desc()
				],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[target_format.into()],
			}),
			primitive: wgpu::PrimitiveState::default(),
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		});


		Self {
			device, queue, target_format,
			num_vertices, vertex_buffer, index_buffer, num_indices,
			render_pipeline,
			camera_uniform, camera_buffer, camera_bind_group,
			uniforms, uniform_buffer, uniform_bind_group
		}
	}


	fn update(&mut self, app: &App) {
		self.camera_uniform.update(&app.camera.as_ref().unwrap(), &app.projection.as_ref().unwrap());
		self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
		self.uniforms.update(&app.time.as_ref().unwrap());
		self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
	}


	fn render(
		&mut self,
		surface_state: &SurfaceState,
		app: &App
	) -> Result<(), wgpu::SurfaceError> {

		self.update(app);

		let output = surface_state.surface.get_current_texture()?;
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
							load: wgpu::LoadOp::Clear(app.clear_color),
							store: true
						}
					}
				],
				depth_stencil_attachment: None
			});

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
			render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}

}


struct AppInner {
	instance: wgpu::Instance,
	adapter: Option<wgpu::Adapter>,
	surface_state: Option<SurfaceState>,
	render_state: Option<RenderState>
}

struct App {
	inner: Arc<RwLock<AppInner>>,

	clear_color: wgpu::Color,
	camera: Option<camera::Camera>,
	projection: Option<camera::Projection>,
	camera_controller: Option<camera::CameraController>,
	time: Option<Duration>
}

impl App {
	fn new(
		instance: wgpu::Instance,
		clear_color: wgpu::Color
	) -> Self {

		let camera = Some(camera::Camera::new((3.0, 3.0, 3.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0)));
		let projection = None;
		let camera_controller = Some(camera::CameraController::new(20.0, 1.0));
		let time = Some(Duration::from_secs(1));

		Self {
			inner: Arc::new(RwLock::new(AppInner {
				instance,
				adapter: None,
				surface_state: None,
				render_state: None
			})),
			clear_color,
			camera, projection, camera_controller,
			time
		}
	}
}

impl Deref for App {
	type Target = Arc<RwLock<AppInner>>;
	fn deref(&self) -> &Self::Target { &self.inner }
}



async fn ensure_render_state_for_surface(
	app: &App,
	new_surface_state: &SurfaceState
) {
	let mut app_guard = app.inner.write().expect("Couldn't get app guard");

	if app_guard.adapter.is_none() {
		let adapter = app_guard.instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),	
				compatible_surface: Some(&new_surface_state.surface),
				force_fallback_adapter: false
			}
		).await.expect("Failed to find an appropriate adapter");

		app_guard.adapter = Some(adapter);
	}
	let adapter = app_guard.adapter.as_ref().unwrap();

	if app_guard.render_state.is_none() {
		let swapchain_format = new_surface_state.surface.get_preferred_format(&adapter).expect("Couldn't get preferred swapchain format");

		let render_state = RenderState::new(
			adapter,
			swapchain_format
		).await;
		app_guard.render_state = Some(render_state);
	}
}



async fn resume<T>(
	app: &mut App,
	event_loop: &EventLoopWindowTarget<T>,
) {
	let new_surface_state = SurfaceState::new(&event_loop, &app.read().unwrap().instance);

	pollster::block_on(ensure_render_state_for_surface(app, &new_surface_state));
	app.write().unwrap().surface_state = Some(new_surface_state);

	let guard = app.write().unwrap();
	let render_state = guard.render_state.as_ref().unwrap();
	let surface_state = guard.surface_state.as_ref().unwrap();
	let swapchain_format = render_state.target_format;
	let window_size = surface_state.window.inner_size();
	let adapter = guard.adapter.as_ref().unwrap();

	let config = wgpu::SurfaceConfiguration {
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
		format: surface_state.surface.get_preferred_format(&adapter).unwrap(),
		width: window_size.width,
		height: window_size.height,
		present_mode: wgpu::PresentMode::Fifo
	};
	surface_state.surface.configure(&render_state.device, &config);
	surface_state.window.request_redraw();
	drop(guard);
	app.projection = Some(camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0));
}



async fn run(
	mut app: App,
	event_loop: EventLoop<()>
) {
	let mut last_time_instant = Instant::now();

	event_loop.run(move |event, event_loop, control_flow| {

		*control_flow = ControlFlow::Wait;
		match event {
			Event::NewEvents(winit::event::StartCause::Init) => {
				#[cfg(not(target_os="android"))]
				pollster::block_on(resume(&mut app, event_loop))
			},
			Event::Resumed => {
				pollster::block_on(resume(&mut app, event_loop))
			},
			Event::Suspended => {
				let mut guard = app.write().unwrap();
				guard.render_state = None;
			},
			Event::RedrawRequested(_) => {
				let mut time = app.time.unwrap();
				time += last_time_instant.elapsed();
				last_time_instant = Instant::now();
				println!("{:?}", time);                 // theres a print here, should print for every frame

				let mut guard = app.write().unwrap();
				let guard = &mut *guard;

				if let Some(surface_state) = &guard.surface_state {
					if let Some(render_state) = &mut guard.render_state {
						match render_state.render(&surface_state, &app) {
							Ok(_) => {},
							// Err(wgpu::SurfaceError::Lost) => app.inner.surface_state.resize(app.inner.surface_state.window.inner_size()),
							Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
							Err(e) => error!("{:?}", e)
						}
					}
				}
			},
			// Event::RedrawEventsCleared => {
			// 	let mut guard = app.read().unwrap();
			// 	guard.surface_state.as_ref().unwrap().window.request_redraw();
			// },
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => {}
		}
	});

}



fn _main() {

	let event_loop = EventLoop::new();

	let instance = wgpu::Instance::new(wgpu::Backends::all());
	let mut app = App::new(instance, wgpu::Color::BLUE);

	pollster::block_on(run(app, event_loop));

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
	simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Error).init().unwrap();

	_main();
}