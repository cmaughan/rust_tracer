mod utils;
mod render;
mod blocks;

use minifb::{Key, Window, WindowOptions};
use utils::*;
use std::error::Error;

use blocks::*;

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
    let mut buffer : Vec<PackedColor> = vec![0xff00ffff; WIDTH * HEIGHT];

    crossbeam::scope(|s| {
        let width = window.get_size().0 as u32;
        let height = window.get_size().1 as u32;
        let mut block_manager = BlockManager::new(width, height);
        let worker = Arc::new(render::Renderer::new(width, height));

        // Create the worker thread
        let thread_worker = worker.clone();
        s.spawn(move |_| {
            thread_worker.render_frame();
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {

            // Send the next few blocks to the renderer
            while let Some(block) = block_manager.next_block() {
                worker.sender.send(block.clone()).unwrap();
            }

            // Get any finished blocks
            let render_results = &worker.poll();

            // Update them
            let has_changed = !render_results.is_empty();
            for result in render_results {
                let block = result.lock().unwrap();
                for i in 0..block.pixels.len() {
                    let color = block.pixels[i];
                    let x = block.rect.x + (i as u32 % rect_width(&block.rect));
                    let y = block.rect.y + (i as u32 / rect_width(&block.rect));
                    let index = index_from_xy(width, height, x, y);
                    buffer[index] = packed_color_from_color(color);
                }

                // Return the block to the renderer
                block_manager.return_block(result.clone());
            }

            if has_changed {
                window
                    .update_with_buffer(&buffer, WIDTH, HEIGHT)
                    .unwrap();
            }
            else {
                window.update();

            }
        }

        worker.stop();

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
