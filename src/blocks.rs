use std::collections::HashSet;
use std::hash::Hash;
use crate::utils::*;

const BLOCK_WIDTH: usize = 32;
const BLOCK_HEIGHT: usize = 32;

/// A Block of pixels filled by the renderer.
/// Typed because the rendering is at float precision, and the packed colors are returned
#[derive(Clone)]
pub struct Block {
    pub rect: Rect,
    pub pixels: Vec<Color>,
    pub iteration: u64
}

/// Allocate a new block for a given rectangle, initializing the pixels to a default value
impl Block {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: *rect,
            pixels: vec![Default::default(); (rect_width(rect) * rect_height(rect)) as usize],
            iteration: 0,
        }
    }
}

pub type ColorBlock = Arc<Mutex<Block>>;

#[derive(Clone)]
pub struct BlockManager {
    pub blocks: Vec<ColorBlock>,
    pub active_blocks: HashSet<ColorBlock>,
    pub max_blocks: usize
}

impl BlockManager {
    pub fn new(width: u32, height: u32) -> BlockManager {
        BlockManager {
            blocks: BlockManager::make_blocks(width, height),
            active_blocks: HashSet::new(),
            max_blocks: num_cpus::get()
        }
    }

    /// Make an array of blocks inside this larger window.
    fn make_blocks(width: u32, height: u32) -> Vec<ColorBlock> {
        let mut blocks = Vec::new();
        for y in (0..height).step_by(BLOCK_HEIGHT as usize) {
            for x in (0..width).step_by(BLOCK_WIDTH as usize) {
                let rc = Rect::new(x, y, std::cmp::min(x + BLOCK_WIDTH as u32, width),
                                   std::cmp::min(y + BLOCK_HEIGHT as u32, height));
                blocks.push(Arc::new(Mutex::new(Block::new(&rc))));
            };
        };
        blocks
    }

    pub fn return_block(&mut self, block: ColorBlock) {
        self.blocks.push(block);
        self.active_blocks.remove(block);
    }

    pub fn next_block(&mut self) -> Option<ColorBlock> {
        if self.blocks.is_empty() || self.active_blocks.len() >= self.max_blocks {
            return None;
        }

        let b = self.blocks.pop().unwrap();
        self.active_blocks.insert(b.clone());
        Some(b)
    }
}


