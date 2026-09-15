// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>, // ARGB values
}

static FONT_DATA: [(char, [u8; 5]); 42] = [
    (' ', [0x00, 0x00, 0x00, 0x00, 0x00]),
    ('A', [0x7E, 0x11, 0x11, 0x11, 0x7E]),
    ('B', [0x7F, 0x49, 0x49, 0x49, 0x36]),
    ('C', [0x3E, 0x41, 0x41, 0x41, 0x22]),
    ('D', [0x7F, 0x41, 0x41, 0x22, 0x1C]),
    ('E', [0x7F, 0x49, 0x49, 0x49, 0x41]),
    ('F', [0x7F, 0x09, 0x09, 0x09, 0x01]),
    ('G', [0x3E, 0x41, 0x49, 0x49, 0x7A]),
    ('H', [0x7F, 0x08, 0x08, 0x08, 0x7F]),
    ('I', [0x00, 0x41, 0x7F, 0x41, 0x00]),
    ('J', [0x20, 0x40, 0x41, 0x3F, 0x01]),
    ('K', [0x7F, 0x08, 0x14, 0x22, 0x41]),
    ('L', [0x7F, 0x40, 0x40, 0x40, 0x40]),
    ('M', [0x7F, 0x02, 0x0C, 0x02, 0x7F]),
    ('N', [0x7F, 0x04, 0x08, 0x10, 0x7F]),
    ('O', [0x3E, 0x41, 0x41, 0x41, 0x3E]),
    ('P', [0x7F, 0x09, 0x09, 0x09, 0x06]),
    ('Q', [0x3E, 0x41, 0x51, 0x21, 0x5E]),
    ('R', [0x7F, 0x09, 0x19, 0x29, 0x46]),
    ('S', [0x46, 0x49, 0x49, 0x49, 0x31]),
    ('T', [0x01, 0x01, 0x7F, 0x01, 0x01]),
    ('U', [0x3F, 0x40, 0x40, 0x40, 0x3F]),
    ('V', [0x1F, 0x20, 0x40, 0x20, 0x1F]),
    ('W', [0x7F, 0x20, 0x18, 0x20, 0x7F]),
    ('X', [0x63, 0x14, 0x08, 0x14, 0x63]),
    ('Y', [0x07, 0x08, 0x70, 0x08, 0x07]),
    ('Z', [0x61, 0x51, 0x49, 0x45, 0x43]),
    ('0', [0x3E, 0x51, 0x49, 0x45, 0x3E]),
    ('1', [0x00, 0x42, 0x7F, 0x40, 0x00]),
    ('2', [0x42, 0x61, 0x51, 0x49, 0x46]),
    ('3', [0x21, 0x41, 0x45, 0x4B, 0x31]),
    ('4', [0x18, 0x14, 0x12, 0x7F, 0x10]),
    ('5', [0x27, 0x45, 0x45, 0x45, 0x39]),
    ('6', [0x3C, 0x4A, 0x49, 0x49, 0x30]),
    ('7', [0x01, 0x71, 0x09, 0x05, 0x03]),
    ('8', [0x36, 0x49, 0x49, 0x49, 0x36]),
    ('9', [0x06, 0x49, 0x49, 0x29, 0x1E]),
    (':', [0x00, 0x36, 0x36, 0x00, 0x00]),
    ('-', [0x08, 0x08, 0x08, 0x08, 0x08]),
    ('.', [0x00, 0x60, 0x60, 0x00, 0x00]),
    ('/', [0x20, 0x10, 0x08, 0x04, 0x02]),
    ('+', [0x08, 0x08, 0x3E, 0x08, 0x08]),
];

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0xFF000000; (width * height) as usize], // Solid black
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pixels = vec![0xFF000000; (width * height) as usize];
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, argb: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y * self.width as i32 + x) as usize;
            self.pixels[index] = argb;
        }
    }

    pub fn clear(&mut self, argb: u32) {
        for pixel in self.pixels.iter_mut() {
            *pixel = argb;
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, argb: u32) {
        for i in x..(x + w) {
            self.set_pixel(i, y, argb);
            self.set_pixel(i, y + h - 1, argb);
        }
        for j in y..(y + h) {
            self.set_pixel(x, j, argb);
            self.set_pixel(x + w - 1, j, argb);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, argb: u32) {
        for j in y..(y + h) {
            for i in x..(x + w) {
                self.set_pixel(i, j, argb);
            }
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, argb: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        loop {
            self.set_pixel(x0, y0, argb);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_image(&mut self, x: i32, y: i32, img_pixels: &[u32], img_w: u32, img_h: u32) {
        for j in 0..img_h as i32 {
            for i in 0..img_w as i32 {
                let img_idx = (j * img_w as i32 + i) as usize;
                if img_idx < img_pixels.len() {
                    let pixel = img_pixels[img_idx];
                    if (pixel >> 24) != 0 {
                        self.set_pixel(x + i, y + j, pixel);
                    }
                }
            }
        }
    }

    pub fn draw_string(&mut self, x: i32, y: i32, text: &str, argb: u32) {
        let mut curr_x = x;
        for c in text.chars() {
            let upper = c.to_ascii_uppercase();
            if let Some(font_entry) = FONT_DATA.iter().find(|entry| entry.0 == upper) {
                let pattern = font_entry.1;
                for col in 0..5 {
                    let col_byte = pattern[col];
                    for row in 0..7 {
                        if (col_byte >> row) & 1 == 1 {
                            self.set_pixel(curr_x + col as i32, y + row as i32, argb);
                        }
                    }
                }
            } else {
                self.draw_rect(curr_x, y, 5, 7, argb);
            }
            curr_x += 7; // 5px character + 2px spacing
        }
    }
}
