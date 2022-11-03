use gameboy::gameboy::GameBoy;

use std::time;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize
};

use pixels:: {
    Error,
    Pixels,
    SurfaceTexture,
};

struct App {
    gameboy: GameBoy,
    pixels: Pixels,
    last_render: std::time::Instant,
}

const FPS: u32 = 144;
const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: f64 = 2.;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
        WindowBuilder::new()
            .with_title("GameBoy emulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    
    let mut app = App {
        gameboy: GameBoy::new(),
        pixels,
        last_render: std::time::Instant::now(),
    };
    
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                // Wait to redraw a frame to match the target frame rate.
                if time::Instant::now() - app.last_render > time::Duration::from_secs_f64(1. / FPS as f64) {
                    window.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                // Redraw requested by the OS
                if let Err(e) = app.pixels.render() {
                    println!("pixels.render() failed: {}", e);
                    control_flow.set_exit();
                }

                app.last_render = std::time::Instant::now();

                // Update the gameboy state.
                app.gameboy.cycle(app.pixels.get_frame(), FPS);
            },
            _ => ()
        }
    });
}