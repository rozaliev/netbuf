use std::{mem};
use std::rt::heap;
use std::cell::RefCell;
use std::rc::Rc;

use NetBuf;
use chunk::Chunk;

pub struct Arena {
    ctl: Rc<RefCell<ArenaCtl>>
}

pub struct ArenaCtl {
    mem: *mut u8,
    chunk_size: usize,
    free_chunks: Vec<usize>
}

impl Arena {
    pub fn new(size: usize, chunks: usize) -> Arena {
        let ptr = unsafe { heap::allocate(size*chunks, mem::min_align_of::<u8>()) };

        Arena {
            ctl: Rc::new(RefCell::new(ArenaCtl {
                mem: ptr,
                chunk_size: size,
                free_chunks: range(0,chunks).collect()
            }))
        }
    }

    pub fn new_buf(&mut self) -> NetBuf {
        NetBuf::new(self.ctl.clone())
    }
}

impl ArenaCtl {
    pub fn new_chunk(&mut self) -> Chunk {
        // debug!("new chunk: {:?}", self.free_chunks);
        let offset = self.free_chunks.pop().unwrap() as isize;
        let ptr = unsafe { self.mem.offset(offset*self.chunk_size as isize) };
        Chunk::new(ptr, self.chunk_size)
    }
}

#[cfg(test)]
mod test {
    use super::Arena;

    #[test]
    fn test_creates_buf() {
        let mut arena = Arena::new(2,2);
        let mut buf = arena.new_buf();
        assert_eq!(buf.len(),0);
    }

    #[test]
    fn test_creates_chunks() {
        let mut arena = Arena::new(2,2);
        let chunk = arena.ctl.borrow_mut().new_chunk();
    }
}