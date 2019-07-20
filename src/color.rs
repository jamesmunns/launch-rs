use palette::{Lab, LinSrgb};

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

fn to_float(val: u8) -> f32 {
    val as f32 / 255f32
}

fn rgb_to_lab(red: u8, green: u8, blue: u8) -> Lab {
    LinSrgb::new(to_float(red), to_float(green), to_float(blue)).into()
}

/// Use the 3d nearest neighbor to determine an approximation for RGB colors
pub fn nearest_palette(red: u8, green: u8, blue: u8) -> u8 {
    let mut furthest_val = 0usize;
    let mut furthest_dist = 3f32 * (255f32).powi(2) + 1f32; // all channels

    let color_a = rgb_to_lab(red, green, blue);

    // calculate the LAB distance (delta e)

    for (i, color) in COLOR_PALETTE.iter().enumerate() {
        let color_b = rgb_to_lab(color.red, color.green, color.blue);

        let distance = (color_a.l - color_b.l).powi(2)
            + (color_a.a - color_b.a).powi(2)
            + (color_a.b - color_b.b).powi(2);

        if distance < furthest_dist {
            furthest_dist = distance;
            furthest_val = i;
        }
    }

    return furthest_val as u8;
}


/// Palette table information from http://launchpaddr.com/mk2palette/
pub const COLOR_PALETTE: [RGBColor; 128] = [
    // 0..64
    RGBColor { red: 0x00, green: 0x00, blue: 0x00 },
    RGBColor { red: 0x1c, green: 0x1c, blue: 0x1c },
    RGBColor { red: 0x7c, green: 0x7c, blue: 0x7c },
    RGBColor { red: 0xfc, green: 0xfc, blue: 0xfc },
    RGBColor { red: 0xff, green: 0x4e, blue: 0x48 },
    RGBColor { red: 0xfe, green: 0x0a, blue: 0x00 },
    RGBColor { red: 0x5a, green: 0x00, blue: 0x00 },
    RGBColor { red: 0x18, green: 0x00, blue: 0x02 },
    RGBColor { red: 0xff, green: 0xbc, blue: 0x63 },
    RGBColor { red: 0xff, green: 0x57, blue: 0x00 },
    RGBColor { red: 0x5a, green: 0x1d, blue: 0x00 },
    RGBColor { red: 0x24, green: 0x18, blue: 0x02 },
    RGBColor { red: 0xfd, green: 0xfd, blue: 0x21 },
    RGBColor { red: 0xfd, green: 0xfd, blue: 0x00 },
    RGBColor { red: 0x58, green: 0x58, blue: 0x00 },
    RGBColor { red: 0x18, green: 0x18, blue: 0x00 },
    RGBColor { red: 0x81, green: 0xfd, blue: 0x2b },
    RGBColor { red: 0x40, green: 0xfd, blue: 0x01 },
    RGBColor { red: 0x16, green: 0x58, blue: 0x00 },
    RGBColor { red: 0x13, green: 0x28, blue: 0x01 },
    RGBColor { red: 0x35, green: 0xfd, blue: 0x2b },
    RGBColor { red: 0x00, green: 0xfe, blue: 0x00 },
    RGBColor { red: 0x00, green: 0x58, blue: 0x01 },
    RGBColor { red: 0x00, green: 0x18, blue: 0x00 },
    RGBColor { red: 0x35, green: 0xfc, blue: 0x47 },
    RGBColor { red: 0x00, green: 0xfe, blue: 0x00 },
    RGBColor { red: 0x00, green: 0x58, blue: 0x01 },
    RGBColor { red: 0x00, green: 0x18, blue: 0x00 },
    RGBColor { red: 0x32, green: 0xfd, blue: 0x7f },
    RGBColor { red: 0x00, green: 0xfd, blue: 0x3a },
    RGBColor { red: 0x01, green: 0x58, blue: 0x14 },
    RGBColor { red: 0x00, green: 0x1c, blue: 0x0e },
    RGBColor { red: 0x2f, green: 0xfc, blue: 0xb1 },
    RGBColor { red: 0x00, green: 0xfb, blue: 0x91 },
    RGBColor { red: 0x01, green: 0x57, blue: 0x32 },
    RGBColor { red: 0x01, green: 0x18, blue: 0x10 },
    RGBColor { red: 0x39, green: 0xbe, blue: 0xff },
    RGBColor { red: 0x00, green: 0xa7, blue: 0xff },
    RGBColor { red: 0x01, green: 0x40, blue: 0x51 },
    RGBColor { red: 0x00, green: 0x10, blue: 0x18 },
    RGBColor { red: 0x41, green: 0x86, blue: 0xff },
    RGBColor { red: 0x00, green: 0x50, blue: 0xff },
    RGBColor { red: 0x01, green: 0x1a, blue: 0x5a },
    RGBColor { red: 0x01, green: 0x06, blue: 0x19 },
    RGBColor { red: 0x47, green: 0x47, blue: 0xff },
    RGBColor { red: 0x00, green: 0x00, blue: 0xfe },
    RGBColor { red: 0x00, green: 0x00, blue: 0x5a },
    RGBColor { red: 0x00, green: 0x00, blue: 0x18 },
    RGBColor { red: 0x83, green: 0x47, blue: 0xff },
    RGBColor { red: 0x50, green: 0x00, blue: 0xff },
    RGBColor { red: 0x16, green: 0x00, blue: 0x67 },
    RGBColor { red: 0x0a, green: 0x00, blue: 0x32 },
    RGBColor { red: 0xff, green: 0x48, blue: 0xfe },
    RGBColor { red: 0xff, green: 0x00, blue: 0xfe },
    RGBColor { red: 0x5a, green: 0x00, blue: 0x5a },
    RGBColor { red: 0x18, green: 0x00, blue: 0x18 },
    RGBColor { red: 0xfb, green: 0x4e, blue: 0x83 },
    RGBColor { red: 0xff, green: 0x07, blue: 0x53 },
    RGBColor { red: 0x5a, green: 0x02, blue: 0x1b },
    RGBColor { red: 0x21, green: 0x01, blue: 0x10 },
    RGBColor { red: 0xff, green: 0x19, blue: 0x01 },
    RGBColor { red: 0x9a, green: 0x35, blue: 0x00 },
    RGBColor { red: 0x7a, green: 0x51, blue: 0x01 },
    RGBColor { red: 0x3e, green: 0x65, blue: 0x00 },

    // 64..128
    RGBColor { red: 0x01, green: 0x38, blue: 0x00 },
    RGBColor { red: 0x00, green: 0x54, blue: 0x32 },
    RGBColor { red: 0x00, green: 0x53, blue: 0x7f },
    RGBColor { red: 0x00, green: 0x00, blue: 0xfe },
    RGBColor { red: 0x01, green: 0x44, blue: 0x4d },
    RGBColor { red: 0x1a, green: 0x00, blue: 0xd1 },
    RGBColor { red: 0x7c, green: 0x7c, blue: 0x7c },
    RGBColor { red: 0x20, green: 0x20, blue: 0x20 },
    RGBColor { red: 0xff, green: 0x0a, blue: 0x00 },
    RGBColor { red: 0xba, green: 0xfd, blue: 0x00 },
    RGBColor { red: 0xac, green: 0xec, blue: 0x00 },
    RGBColor { red: 0x56, green: 0xfd, blue: 0x00 },
    RGBColor { red: 0x00, green: 0x88, blue: 0x00 },
    RGBColor { red: 0x01, green: 0xfc, blue: 0x7b },
    RGBColor { red: 0x00, green: 0xa7, blue: 0xff },
    RGBColor { red: 0x02, green: 0x1a, blue: 0xff },
    RGBColor { red: 0x35, green: 0x00, blue: 0xff },
    RGBColor { red: 0x78, green: 0x00, blue: 0xff },
    RGBColor { red: 0xb4, green: 0x17, blue: 0x7e },
    RGBColor { red: 0x41, green: 0x20, blue: 0x00 },
    RGBColor { red: 0xff, green: 0x4a, blue: 0x01 },
    RGBColor { red: 0x82, green: 0xe1, blue: 0x00 },
    RGBColor { red: 0x66, green: 0xfd, blue: 0x00 },
    RGBColor { red: 0x00, green: 0xfe, blue: 0x00 },
    RGBColor { red: 0x00, green: 0xfe, blue: 0x00 },
    RGBColor { red: 0x45, green: 0xfd, blue: 0x61 },
    RGBColor { red: 0x01, green: 0xfb, blue: 0xcb },
    RGBColor { red: 0x50, green: 0x86, blue: 0xff },
    RGBColor { red: 0x27, green: 0x4d, blue: 0xc8 },
    RGBColor { red: 0x84, green: 0x7a, blue: 0xed },
    RGBColor { red: 0xd3, green: 0x0c, blue: 0xff },
    RGBColor { red: 0xff, green: 0x06, blue: 0x5a },
    RGBColor { red: 0xff, green: 0x7d, blue: 0x01 },
    RGBColor { red: 0xb8, green: 0xb1, blue: 0x00 },
    RGBColor { red: 0x8a, green: 0xfd, blue: 0x00 },
    RGBColor { red: 0x81, green: 0x5d, blue: 0x00 },
    RGBColor { red: 0x3a, green: 0x28, blue: 0x02 },
    RGBColor { red: 0x0d, green: 0x4c, blue: 0x05 },
    RGBColor { red: 0x00, green: 0x50, blue: 0x37 },
    RGBColor { red: 0x13, green: 0x14, blue: 0x29 },
    RGBColor { red: 0x10, green: 0x1f, blue: 0x5a },
    RGBColor { red: 0x6a, green: 0x3c, blue: 0x18 },
    RGBColor { red: 0xac, green: 0x04, blue: 0x01 },
    RGBColor { red: 0xe1, green: 0x51, blue: 0x36 },
    RGBColor { red: 0xdc, green: 0x69, blue: 0x00 },
    RGBColor { red: 0xfe, green: 0xe1, blue: 0x00 },
    RGBColor { red: 0x99, green: 0xe1, blue: 0x01 },
    RGBColor { red: 0x60, green: 0xb5, blue: 0x00 },
    RGBColor { red: 0x1b, green: 0x1c, blue: 0x31 },
    RGBColor { red: 0xdc, green: 0xfd, blue: 0x54 },
    RGBColor { red: 0x76, green: 0xfb, blue: 0xb9 },
    RGBColor { red: 0x96, green: 0x98, blue: 0xff },
    RGBColor { red: 0x8b, green: 0x62, blue: 0xff },
    RGBColor { red: 0x40, green: 0x40, blue: 0x40 },
    RGBColor { red: 0x74, green: 0x74, blue: 0x74 },
    RGBColor { red: 0xde, green: 0xfc, blue: 0xfc },
    RGBColor { red: 0xa2, green: 0x04, blue: 0x01 },
    RGBColor { red: 0x34, green: 0x01, blue: 0x00 },
    RGBColor { red: 0x00, green: 0xd2, blue: 0x01 },
    RGBColor { red: 0x00, green: 0x41, blue: 0x01 },
    RGBColor { red: 0xb8, green: 0xb1, blue: 0x00 },
    RGBColor { red: 0x3c, green: 0x30, blue: 0x00 },
    RGBColor { red: 0xb4, green: 0x5d, blue: 0x00 },
    RGBColor { red: 0x4c, green: 0x13, blue: 0x00 },
];
