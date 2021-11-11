mod utils;
mod render;

use minifb::{Key, Window, WindowOptions};
use utils::*;
use std::error::Error;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() -> Result<(), Box<dyn Error>> {
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
    let buffer : Vec<PackedColor> = vec![0xff00ffff; WIDTH * HEIGHT];

    crossbeam::scope(|s| {
        let worker = render::Renderer::new(window.get_size().0 as u32, window.get_size().1 as u32);
        s.spawn(move |_| {
            worker.render_frame();
        });
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(0, 0);
    }
}
