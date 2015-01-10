use std::{mem};
use std::rt::heap;
use std::cell::RefCell;
use std::rc::Rc;

use NetBuf;
use chunk::Chunk;

#[derive(Clone)]
struct ChunkToken {
    holder: usize,
    offset: usize
}

pub struct Arena {
    ctl: Rc<RefCell<ArenaCtl>>
}

pub struct ArenaCtl {
    chunks_holder: Vec<*mut u8>,
    chunk_size: usize,
    chunks_per_holder: usize,
    free_chunks: Vec<ChunkToken>
}

impl Arena {
    pub fn new(size: usize, chunks: usize) -> Arena {
        Arena {
            ctl: Rc::new(RefCell::new(ArenaCtl::new(size, chunks)))
        }
    }

    pub fn new_buf(&mut self) -> NetBuf {
        NetBuf::new(self.ctl.clone())
    }
}

impl ArenaCtl {
    pub fn new(size: usize, chunks: usize) -> ArenaCtl {
        ArenaCtl {
            chunks_holder: vec!(),
            chunk_size: size,
            chunks_per_holder: chunks,
            free_chunks: vec!()
        }
    }

    pub fn new_chunk(&mut self) -> Chunk {
        if self.free_chunks.len() == 0 {
            self.allocate_chunks_holder();
        }

        let token = self.free_chunks.pop().expect("NetBuf: there is no free chunks");
        let ptr = unsafe { self.chunks_holder[token.holder].offset((token.offset*self.chunk_size) as isize) };
        Chunk::new(ptr, self.chunk_size)
    }

    fn allocate_chunks_holder(&mut self) {
        let ptr = unsafe { heap::allocate(self.chunk_size*self.chunks_per_holder, mem::min_align_of::<u8>()) };
        self.chunks_holder.push(ptr);

        let holder_idx = self.chunks_holder.len()-1;
        let ids: Vec<ChunkToken> = range(0,self.chunks_per_holder).map(
            |i| ChunkToken { holder: holder_idx, offset: i }
        ).collect();

        self.free_chunks.push_all(ids.as_slice());
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

    #[test]
    fn test_allocates_if_full() {
        let mut arena = Arena::new(2,1);
        let mut buf = arena.new_buf();
        buf.write(b"asdf");
        buf.write(b"qwerty");
        assert_eq!(buf.len(),10);

        let mut data = Vec::with_capacity(20);
        buf.write_to(&mut data);

        assert_eq!(data, b"asdfqwerty");
    }
}