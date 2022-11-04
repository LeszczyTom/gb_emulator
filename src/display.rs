use std::time::Instant;

use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
    dpi::{LogicalSize, PhysicalPosition}
};

use pixels:: {
    Pixels,
    SurfaceTexture,
};

pub struct Display {
    window: Window,
    pixels: Pixels,
    _width: u32,
    _height: u32,
    _scale: f64,
    fps: u32,
    last_render: Instant,
}

impl Display {
    pub fn new(width: u32, height: u32, scale: f64, fps: u32, title: &str, spawn_position: PhysicalPosition<u16>, eventloop: &EventLoop<()>) -> Self {
        let size = LogicalSize::new(width as f64, height as f64);
        let scaled_size = LogicalSize::new(width as f64 * scale, height as f64 * scale);

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_max_inner_size(scaled_size)
            .with_resizable(false)
            .with_maximized(false)
            .with_position(spawn_position)
            .build(&eventloop)
            .unwrap();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let pixels = match Pixels::new(width, height, surface_texture) {
            Ok(pixels) => pixels,
            Err(e) => panic!("Error creating window: {}", e),
        };

        Self {
            window,
            pixels,
            _width: width,
            _height: height,
            _scale: scale,
            fps,
            last_render: Instant::now(),
        }
    }

    pub fn is_ready_to_render(&self) -> bool {
        Instant::now() - self.last_render > std::time::Duration::from_secs_f64(1. / self.fps as f64)
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn render(&mut self) {
        if let Err(e) = self.pixels.render() {
            panic!("Error rendering pixels: {}", e);
        }

        self.last_render = Instant::now();
    }

    pub fn get_mut_pixels(&mut self) -> &mut Pixels {
        &mut self.pixels
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }
}