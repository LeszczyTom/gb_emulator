use gameboy::gameboy::GameBoy;

use pixels:: {
    Error,
    Pixels,
    SurfaceTexture,
};

use game_loop::{game_loop, Time, TimeTrait};
use game_loop::winit::event::{Event, WindowEvent};
use game_loop::winit::event_loop::EventLoop;
use game_loop::winit::window::WindowBuilder;

use winit::dpi::LogicalSize;

struct App {
    gameboy: GameBoy,
    pixels: Pixels,
    paused: bool,
}

const FPS: u32 = 240;
const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 5.0, HEIGHT as f64 * 5.0);
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
    
    
    let app = App {
        gameboy: GameBoy::new(),
        pixels,
        paused: false,
    };

    game_loop(event_loop, window, app, FPS, 0.1, move |g| {

        g.game.gameboy.cycle(g.game.pixels.get_frame(), FPS);

    }, move |g| {
        if let Err(e) = g.game.pixels.render() {
            println!("pixels.render() failed: {}", e);
            g.exit();
        }
    }, |g, event| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                g.exit();
            },
            Event::WindowEvent { 
                window_id: _, 
                event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } 
            } => {
                println!("Keyboard input!: {:?}", input);
            },
            _ => {}
        }
    });
}