mod utils;

use minifb::{Key, Window, WindowOptions};
use utils::*;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut window = Window::new(
        "Path Tracing",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Storage space
    let buffer : Vec<PackedColor> = vec![0; WIDTH * HEIGHT];

    // Window loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(0, 0);
    }
}
