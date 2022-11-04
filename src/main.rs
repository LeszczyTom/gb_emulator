mod display;
mod debug;

use std::collections::HashMap;

use display::Display;
use gameboy::gameboy::GameBoy;

use winit::{
    event::{ Event, WindowEvent },
    event_loop::EventLoop
};

use pixels::Error;

#[derive(Eq, Hash, PartialEq)]
enum DisplayType {
    GameBoy,
    TileSetDebug,
    BackgroundDebug
}

struct App {
    gameboy: GameBoy,
    displays: HashMap<DisplayType, Display>
}

fn main() -> Result<(), Error> {
    let mut app = App {
        gameboy: GameBoy::new(),
        displays: HashMap::new()
    };

    let event_loop = EventLoop::new();

    app.displays.insert(
        DisplayType::GameBoy,
        Display::new(160, 144, 3., 144, "GameBoy emulator", &event_loop)
    );
    
    app.displays.insert(
        DisplayType::TileSetDebug,
        Display::new(128, 192, 2., 60, "TileSet Debug", &event_loop)
    );
    
    app.displays.insert(
        DisplayType::BackgroundDebug,
        Display::new(256, 256, 2., 60, "Background Debug", &event_loop)
    );

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } => {
                // Remove the display from the hashmap
                app.displays.retain(|_, display| display.get_window().id() != window_id);

                // If main window is closed, exit the program
                match app.displays.get(&DisplayType::GameBoy) {
                    Some(_) => (),
                    None => control_flow.set_exit() // TODO: Panic everytime, might find a better way to do this
                }

                // If no more displays, exit
                if app.displays.is_empty() {
                    control_flow.set_exit();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ }, 
                ..
            } => {
                println!("Keyboard input: {:?}", input);
            },
            Event::MainEventsCleared => {
                // Wait to redraw a frame to match the target frame rate.
                let display = match app.displays.get_mut(&DisplayType::GameBoy) {
                    Some(display) => display,
                    None => panic!("No main display found")
                };
                if display.is_ready_to_render() {
                    display.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                // Redraw requested by the OS
                for (display_type, display) in app.displays.iter_mut() {
                    match display_type {
                        DisplayType::GameBoy => {
                            // update gameboy
                            let fps = display.get_fps();
                            app.gameboy.cycle(display.get_mut_pixels().get_frame(), fps);
                        },
                        DisplayType::TileSetDebug => {
                            // update tileset debug
                            debug::update_tileset_debug(&mut app.gameboy, display.get_mut_pixels().get_frame());
                        },
                        DisplayType::BackgroundDebug => {
                            // update background debug
                            debug::update_background_debug(&mut app.gameboy, display.get_mut_pixels().get_frame());
                        }
                    }
                }

                for display in app.displays.values_mut() {
                    display.render();
                }
            },
            _ => ()
        }
    });
}