use scoped_threadpool::Pool;
use crossbeam::atomic::AtomicCell;
use crossbeam_channel::bounded;
use std::time::Duration;

use crate::utils::*;
use crate::blocks::*;

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
pub struct Renderer {
    pub sender: crossbeam_channel::Sender<ColorBlock>,
    receiver: crossbeam_channel::Receiver<ColorBlock>,
    stop: AtomicCell<bool>,
    finished: AtomicCell<bool>,
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
        }
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

    /*pub fn add_blocks(&self) {

    }
    */

    pub fn render_frame(&self) {
        self.finished.store(false);

        let mut threadpool = Pool::new(num_cpus::get() as u32);
        threadpool.scoped(|scoped| {
            let d = Duration::from_millis(10);
            loop {
                if !self.stop.load() {
                    let b = self.receiver.recv_timeout(d);
                    if b.is_err() {
                        continue;
                    }

                    let block = b.unwrap();

                    // Run a thread to render this block
                    scoped.execute(move || {
                        // Lock and render the block
                        self.render_block(&mut block.lock().unwrap());
                        self.sender.send(block.clone()).unwrap();
                    });
                }
            }
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

    /*
    pub fn finished(&self) -> bool {
        return self.finished.load();
    }
     */

    pub fn stop(&self) {
        self.stop.store(true);

        // Drain channel messages
        while !self.receiver.is_empty() {
            let _ = self.receiver.recv();
        }
    }
}
