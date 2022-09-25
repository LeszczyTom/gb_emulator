pub mod gmb;

use std::time::Duration;

use pixels:: {
    Error,
    Pixels,
    SurfaceTexture,
};

use game_loop::{game_loop, Time, TimeTrait};
use game_loop::winit::event::{Event, WindowEvent};
use game_loop::winit::event_loop::EventLoop;
use game_loop::winit::window::WindowBuilder;

use gmb::GMB;

use winit::dpi::LogicalSize;

struct App {
    gmb: GMB,
    pixels: Pixels,
}

const FPS: u64 = 60;
const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
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
    
    let mut gmb = GMB::new();
    gmb.init("resources/tetris.gb");
    gmb.ppu.set_pixel_at(20, 20, [0x48, 0xb2, 0xe8, 0xff]);

    let app = App {
        gmb,
        pixels,
    };

    game_loop(event_loop, window, app, FPS as u32, 0.5, move |g| {
        // update
        g.game.gmb.cycle();
    }, move |g| {
        // draw
        g.game.gmb.ppu.draw(g.game.pixels.get_frame());
        println!("{}", g.game.gmb.cpu_debug());
        if let Err(e) = g.game.pixels.render() {
            println!("pixels.render() failed: {}", e);
            g.exit();
        }
        
        let dt = Duration::from_nanos(1_000_000_000 / FPS).as_secs_f64() - Time::now().sub(&g.current_instant());
        if dt > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(dt));
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