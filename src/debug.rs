use gameboy::gameboy::GameBoy;

pub fn update_tileset_debug(gameboy: &mut GameBoy, frame: &mut [u8]) {
    let tiles_per_line = 16;
    let tile_width = 8;

    let height = 192; 
    let width = tiles_per_line * tile_width;
    let tiles: usize = tiles_per_line * (height / tile_width);
    
    for i in 0..tiles {
        let tile = gameboy.get_memory().get_tile(i);
        let x = i % tiles_per_line;
        let y = i / tiles_per_line; 

        paint_tile(tile, x, y, width, frame);
    }
}

pub fn update_background_debug(gameboy: &mut GameBoy, frame: &mut [u8]) {
    let tiles_per_line = 32;
    let tile_width = 8;

    let width = tiles_per_line * tile_width;
    let tiles: usize = tiles_per_line * tiles_per_line;

    for tile in 0..tiles {
        let x = tile % tiles_per_line;
        let y = tile / tiles_per_line;

        let tile_id = gameboy.get_memory().get_background_tile_id(y * tiles_per_line + x) as usize; 
        let tile = gameboy.get_memory().get_tile(tile_id);

        paint_tile(tile, x, y, width,  frame);
    }

    paint_viewport_border(gameboy.get_memory().get_scy() as usize, gameboy.get_memory().get_scx() as usize, frame);
}

fn paint_tile(tile: [u8; 16], x: usize, y: usize, width: usize, frame: &mut [u8]) {
    for i in (0..15).step_by(2) {
        let byte = tile[i as usize];
        let byte1 = tile[i as usize + 1];

        // Construct the 8 pixels of the line and write them to the frame
        for j in 0..8 {
            let low = (byte >> (7 - j)) & 1; 
            let high = (byte1 >> (7 - j)) & 1; 
            let color = match high << 1 | low { // 0b00 = white, 0b01 = light gray, 0b10 = dark gray, 0b11 = black
                0 => [0xff, 0xff, 0xff, 0xff], // white
                1 => [0xaa, 0xaa, 0xaa, 0xff], // light gray
                2 => [0x55, 0x55, 0x55, 0xff], // dark gray
                3 => [0x00, 0x00, 0x00, 0xff], // black
                _ => panic!("Invalid color")
            };
            
            let px = 8 * x + j; // x position of the pixel
            let py = 8 * y + i as usize / 2; // y position of the pixel
            let index = (py * width + px) * 4; // index of the pixel in the frame
            frame[index] = color[0];        // R
            frame[index + 1] = color[1];    // G
            frame[index + 2] = color[2];    // B
            frame[index + 3] = color[3];    // A
        }
    }
}

fn paint_viewport_border(scy: usize, scx: usize, frame: &mut [u8]) {
    let viewport_width = 160;
    let viewport_height = 144;
    let size = 256;
    let color: u32 = 0xff0000ff; // red

    let top_left = (scy * size + scx) * 4;
    let bottom_left = ((scy + viewport_height) * size + scx) * 4;
    
    for i in 0..viewport_width {
        // top border
        let index = (top_left + i * 4) as usize;
        paint_pixel(color, index, frame);

        // bottom border
        let index = bottom_left + i * 4;
        paint_pixel(color, index, frame);
    }

    for i in 0..viewport_height {
        // left border
        let index = (top_left + i * size * 4) as usize;
        paint_pixel(color, index, frame);

        // right border
        let index = (top_left + i * size * 4 + viewport_width * 4) as usize;
        paint_pixel(color, index, frame);
    }
}

fn paint_pixel(color: u32, index: usize, frame: &mut [u8]) {
    frame[index] = (color & 0xFF) as u8;
    frame[index + 1] = (color >> 8 & 0xFF) as u8 ;
    frame[index + 2] = (color >> 16 & 0xFF) as u8;
    frame[index + 3] = (color >> 24) as u8;
}