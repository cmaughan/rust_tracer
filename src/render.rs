use scoped_threadpool::Pool;
use crossbeam::atomic::AtomicCell;
use crossbeam_channel::bounded;

use crate::utils::*;

const BLOCK_WIDTH: usize = 32;
const BLOCK_HEIGHT: usize = 32;

/// A Block of pixels filled by the renderer.
/// Typed because the rendering is at float precision, and the packed colors are returned
#[derive(Clone)]
pub struct Block<T> {
    pub rect: Rect,
    pub pixels: Vec<T>,
    pub iteration: u64
}

/// Allocate a new block for a given rectangle, initializing the pixels to a default value
impl<T> Block<T> where T : Clone + Default {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: *rect,
            pixels: vec![Default::default(); (rect_width(rect) * rect_height(rect)) as usize],
            iteration: 0
        }
    }
}

// A high precision block
pub type ColorBlock = Block<Color>;

// An output color block
//pub type OutputBlock = Block<PackedColor>;

pub struct Renderer {
    sender: crossbeam_channel::Sender<ColorBlock>,
    receiver: crossbeam_channel::Receiver<ColorBlock>,
    stop: AtomicCell<bool>,
    finished: AtomicCell<bool>,
    blocks: Vec<ColorBlock>
}

impl Renderer {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        let (s, r) = bounded(width as usize * height as usize);
        Renderer {
            sender: s,
            receiver: r,
            stop: AtomicCell::new(false),
            finished: AtomicCell::new(true),
            blocks: Renderer::make_blocks::<Color>( width, height)
        }
    }

    /// Make an array of blocks inside this larger window.
    fn make_blocks<T>(width: u32, height: u32) -> Vec<Block<T>>
        where T: Clone + Default {
        let mut blocks: Vec<Block<T>> = Vec::new();
        for y in (0..height).step_by(BLOCK_HEIGHT as usize) {
            for x in (0..width).step_by(BLOCK_WIDTH as usize) {
                blocks.push(Block::<T>::new(&Rect::new(x, y, std::cmp::min(x + BLOCK_WIDTH as
                    u32, width), std::cmp::min(y + BLOCK_HEIGHT as u32, height))));
            };
        };
        blocks
    }

    pub fn render_frame(&self) {
        let mut threadpool = Pool::new(num_cpus::get() as u32);
        self.finished.store(false);
        threadpool.scoped(|scoped| {
            for block in &self.blocks {
                let mut new_block: ColorBlock = ColorBlock::new(&block.rect);
                scoped.execute(move || {
                    let mut rng = FastRepRand::new(rand::random());
                    if !self.stop.load() {
                        let block_width = rect_width(&block.rect);
                        let block_height = rect_height(&block.rect);

                        for y in 0..block_height {
                            for x in 0..block_width {
                                let index: usize = (y * block_width + x) as usize;
                                new_block.pixels[index] = color_random(&mut rng);
                            };
                        };

                        // Now we have finished the block, send it.
                        if !self.stop.load() {
                            self.sender.send(new_block).unwrap();
                        }
                    }
                });
            }
            // Wait for all blocks to finish drawing
            scoped.join_all();
            self.finished.store(true);
        });
    }

    /// Returns fully rendered pixels in the channel
    pub fn poll(&self) -> Vec<ColorBlock> {
        let mut results = Vec::new();
        while !self.receiver.is_empty() {
            let res = self.receiver.recv().unwrap();
            results.push(res);
        }
        results
    }

    pub fn finished(&self) -> bool {
        return self.finished.load();
    }

    pub fn stop(&self) {
        self.stop.store(true);

        // Drain channel messages
        while !self.receiver.is_empty() {
            let _ = self.receiver.recv();
        }
    }
}
