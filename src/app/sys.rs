use std::time::Instant;

use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};

use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

const INITIAL_WINDOW_WIDTH: f64 = 1024.0;
const INITIAL_WINDOW_HEIGHT: f64 = 768.0;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32
}

impl System {
    pub fn new(title: &str) -> System {
        // Initialize Window
        let window_event_loop = EventLoop::new();
        let window_context = glutin::ContextBuilder::new().with_vsync(true);
        let window_builder = WindowBuilder::new()
            .with_title(title.to_owned())
            .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT));

        // Initialize Display
        let display = Display::new(window_builder, window_context, &window_event_loop)
            .expect("Failed to initialize the display");

        // Initialize GUI context
        let mut imgui_context = Context::create();
        imgui_context.set_ini_filename(None);

        // Initialize Platform specific context
        let mut platform = WinitPlatform::init(&mut imgui_context); {
            let gl_window_context = display.gl_window();
            let window = gl_window_context.window();

            platform.attach_window(imgui_context.io_mut(), &window, HiDpiMode::Rounded);
        };

        // Initialize GUI font
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;

        imgui_context.fonts()
            .add_font(&[
                FontSource::DefaultFontData {
                    config: Some(FontConfig {
                        size_pixels: font_size,
                        ..FontConfig::default()
                    })
                }
            ]);

        imgui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        // Initialize GUI renderer
        let renderer = Renderer::init(&mut imgui_context, &display)
            .expect("Failed to initialize display renderer");

        System {
            event_loop: window_event_loop,
            display: display,
            imgui: imgui_context,
            platform: platform,
            renderer: renderer,
            font_size: font_size
        }
    }

    pub fn start_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let mut last_frame = Instant::now();
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), &gl_window.window())
                    .expect("Failed to prepare window frame");

                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();
                let mut run = true;

                run_ui(&mut run, &mut ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();

                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());

                let draw_data = ui.render();

                renderer.render(&mut target, draw_data)
                    .expect("Failed to render window frame");

                target.finish()
                    .expect("Failed to swap window buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event)
            }
        })
    }
}