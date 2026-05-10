pub use crate::text::RgbColor;

pub mod parser;
pub mod resolver;

fn parse_hex_color(s: &str) -> Option<RgbColor> {
    let hex = s.strip_prefix('#')?;
    let n = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        6 => Some(RgbColor {
            r: ((n >> 16) & 0xFF) as u8,
            g: ((n >> 8) & 0xFF) as u8,
            b: (n & 0xFF) as u8,
        }),
        3 => {
            let r = ((n >> 8) & 0xF) as u8;
            let g = ((n >> 4) & 0xF) as u8;
            let b = (n & 0xF) as u8;
            Some(RgbColor {
                r: r << 4 | r,
                g: g << 4 | g,
                b: b << 4 | b,
            })
        }
        _ => None,
    }
}
