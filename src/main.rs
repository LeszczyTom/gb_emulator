mod display;

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
        Display::new(128, 192, 3., 60, "TileSet Debug", &event_loop)
    );
    
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
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
                            update_tileset_debug(&mut app.gameboy, display.get_mut_pixels().get_frame());
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

fn update_tileset_debug(gameboy: &mut GameBoy, frame: &mut [u8]) {
    // Tiles at address 0x8000, 384 tiles, 16 bytes per tile
    // 8x8 pixels, 2 bits per pixel, 16 bytes per tile
    let nb_tiles: u16 = 384;
    let tile_size: u16 = 16;
    let pixel_size = 8;
    let width = 128;

    // Go through all tiles
    for i in 0..nb_tiles as u16 {
        let tile = gameboy.get_memory().get_tile(i as usize);
        let x = ((i % tile_size) * 8) as usize;
        let y = ((i / tile_size) * 8) as usize; 

        // Go through all 16 bytes of the tile
        for i in (0..tile_size - 1).step_by(2) {
            let byte = tile[i as usize];
            let byte1 = tile[i as usize + 1];

            // Construct the 8 pixels of the line and write them to the frame
            for j in 0..pixel_size {
                let low = (byte >> (7 - j)) & 1;
                let high = (byte1 >> (7 - j)) & 1;
                let color = match high << 1 | low { // 0b00 = white, 0b01 = light gray, 0b10 = dark gray, 0b11 = black
                    0 => [0xff, 0xff, 0xff, 0xff], // white
                    1 => [0xaa, 0xaa, 0xaa, 0xff], // light gray
                    2 => [0x55, 0x55, 0x55, 0xff], // dark gray
                    3 => [0x00, 0x00, 0x00, 0xff], // black
                    _ => panic!("Invalid color")
                };
                let x = x + j;
                let y = y + i as usize / 2; 
                let index = (y * width + x) * 4;
                frame[index] = color[0];        // R
                frame[index + 1] = color[1];    // G
                frame[index + 2] = color[2];    // B
                frame[index + 3] = color[3];    // A
            }
        }
    }
}