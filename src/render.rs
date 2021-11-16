use scoped_threadpool::Pool;
use crossbeam::atomic::AtomicCell;
use crossbeam_channel::bounded;
use std::sync::Mutex;

use crate::utils::*;

const BLOCK_WIDTH: usize = 32;
const BLOCK_HEIGHT: usize = 32;

/*
// Create a vector of data structures, which will be modified by threads
let n = 50;
let mut entries = Vec::new();
for _ in 0..n {
entries.push(Data { val: 0 });
}

entries.par_iter_mut().for_each(|d| d.val += 1);

// Work on the data after threads are finished
let total: u32 = entries.iter().fold(0, |acc
*/
/// A Block of pixels filled by the renderer.
/// Typed because the rendering is at float precision, and the packed colors are returned
#[derive(Clone)]
pub struct Block {
    pub rect: Rect,
    pub pixels: Vec<Color>,
    pub iteration: u64
}

type ColorBlock = Arc<Mutex<Block>>;

/// Allocate a new block for a given rectangle, initializing the pixels to a default value
impl Block {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: *rect,
            pixels: vec![Default::default(); (rect_width(rect) * rect_height(rect)) as usize],
            iteration: 0
        }
    }
}

pub struct Renderer {
    sender: crossbeam_channel::Sender<ColorBlock>,
    receiver: crossbeam_channel::Receiver<ColorBlock>,
    stop: AtomicCell<bool>,
    finished: AtomicCell<bool>,
    blocks: Vec<ColorBlock>,
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
            blocks: Renderer::make_blocks( width, height),
        }
    }

    /// Make an array of blocks inside this larger window.
    fn make_blocks(width: u32, height: u32) -> Vec<ColorBlock> {
        let mut blocks = Vec::new();
        for y in (0..height).step_by(BLOCK_HEIGHT as usize) {
            for x in (0..width).step_by(BLOCK_WIDTH as usize) {
                blocks.push(Arc::new(Mutex::new(Block::new(&Rect::new(x, y, std::cmp::min(x +
                                                                                              BLOCK_WIDTH as
                                                                                                  u32, width), std::cmp::min(y + BLOCK_HEIGHT as u32, height))))));
            };
        };
        blocks
    }

    pub fn render_block(&self, block : &mut Block) {
        let mut rng = FastRepRand::new(rand::random());
        let block_width = rect_width(&block.rect);
        let block_height = rect_height(&block.rect);

        for y in 0..block_height {
            for x in 0..block_width {
                let index: usize = (y * block_width + x) as usize;
                block.pixels[index] = color_random(&mut
                    rng);
            };
        };
    }

    pub fn render_frame(&self) {
        self.finished.store(false);

        let mut threadpool = Pool::new(num_cpus::get() as u32);
        threadpool.scoped(|scoped| {

            if !self.stop.load() {
                for b in &self.blocks {

                    // Run a thread to render this block
                    scoped.execute(move || {

                        // Lock and render the block
                        self.render_block(&mut b.lock().unwrap());
                        self.sender.send(b.clone()).unwrap();

                    });
                }
            }
            self.finished.store(true);
        });
    }

    /// Returns fully rendered pixel blocks
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
