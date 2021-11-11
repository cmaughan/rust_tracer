use scoped_threadpool::Pool;
use crossbeam::atomic::AtomicCell;
use crossbeam_channel::bounded;

use crate::utils::*;

pub struct OutputBlock {
    pub rect: Rect,
    pub pixels: Vec<PackedColor>
}

pub struct RenderBlock {
    pub rect: Rect,
    pub pixels: Vec<Color>
}

pub struct Renderer {
    width: u32,
    height: u32,
    sender: crossbeam_channel::Sender<OutputBlock>,
    receiver: crossbeam_channel::Receiver<OutputBlock>,
    stop: AtomicCell<bool>
}

impl Renderer {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        let (s, r) = bounded(width as usize * height as usize);
        Renderer {
            width,
            height,
            sender: s,
            receiver: r,
            stop: AtomicCell::new(false)
        }
    }

    pub fn render_frame(&self) {
        let mut threadpool = Pool::new(num_cpus::get() as u32);

        threadpool.scoped(|scoped| {
            (0..self.height).into_iter().for_each(|_line| {
                scoped.execute(move || {
                    if !self.stop.load() {
                        (0..self.width).into_iter().for_each(|index| {
                        }); // for_each pixel

                        /*
                        if !self.stop.load() {
                            //self.sender.send(renderblock).unwrap();
                        }
                        */
                    }
                });
            });
        });
    }

    /// Returns fully rendered pixels in the channel
    /*
    pub fn poll_results(&self) -> Vec<RenderBlock> {
        let mut results = Vec::new();
        let mut limit = self.image_width * self.image_height;
        while !self.channel_receiver.is_empty() {
            let res = self.channel_receiver.recv().unwrap();
            results.push(res);
            limit -= 1;
            if limit == 0 {
                break;
            }
        }
        results
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
