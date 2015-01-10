use std::{mem};
use std::rt::heap;
use std::cell::RefCell;
use std::rc::Rc;

use NetBuf;
use chunk::Chunk;

#[derive(Clone, PartialEq, Show)]
pub struct ChunkToken {
    holder: usize,
    offset: usize
}

pub trait Releasable {
    fn release(&self, token: ChunkToken);
}

#[derive(Clone)]
pub struct Arena {
    ctl: Rc<RefCell<ArenaCtl>>
}


pub struct ArenaCtl {
    chunks_holder: Vec<*mut u8>,
    chunk_size: usize,
    chunks_per_holder: usize,
    free_chunks: Vec<ChunkToken>
}

impl Releasable for Arena {
    fn release(&self, token: ChunkToken){
        self.ctl.borrow_mut().free_chunks.push(token);
    }
}

impl ChunkToken {
    pub fn new(holder: usize, offset: usize) -> ChunkToken {
        ChunkToken {
            holder: holder,
            offset: offset
        }
    }
}

impl Arena {
    pub fn new(size: usize, chunks: usize) -> Arena {
        Arena {
            ctl: Rc::new(RefCell::new(ArenaCtl::new(size, chunks)))
        }
    }

    pub fn new_buf(&self) -> NetBuf {
        NetBuf::new(self.clone())
    }

    pub fn free_chunks(&self) -> usize {
        self.ctl.borrow().free_chunks.len()
    }

    pub fn new_chunk(&self) -> Chunk<Arena> {
        let mut ctl = self.ctl.borrow_mut();

        if ctl.free_chunks.len() == 0 {
            ctl.allocate_chunks_holder();
        }

        let token = ctl.free_chunks.pop().expect("NetBuf: there is no free chunks");
        let ptr = unsafe { ctl.chunks_holder[token.holder].offset((token.offset*ctl.chunk_size) as isize) };
        Chunk::new(ptr, ctl.chunk_size, token, self.clone())
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
    use super::{Arena,ChunkToken,Releasable};

    #[test]
    fn test_creates_buf() {
        let mut arena = Arena::new(2,2);
        let mut buf = arena.new_buf();
        assert_eq!(buf.len(),0);
        assert_eq!(arena.free_chunks(),0)
    }

    #[test]
    fn test_creates_chunks() {
        let mut arena = Arena::new(2,3);
        let chunk = arena.new_chunk();
        assert_eq!(arena.free_chunks(),2)
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

        assert_eq!(buf.len(),0);
        assert_eq!(arena.free_chunks(),5)
    }

    #[test]
    fn test_is_releasable() {
        let mut arena = Arena::new(2,1);
        assert_eq!(arena.free_chunks(),0);
        arena.release(ChunkToken::new(0,0));
        assert_eq!(arena.free_chunks(),1);
        arena.release(ChunkToken::new(1,1));
        assert_eq!(arena.free_chunks(),2);
    }

    #[test]
    fn test_frees_space_on_chunk_drop() {
        let mut arena = Arena::new(2,2);
        assert_eq!(arena.free_chunks(),0);
        let chunk = arena.new_chunk();
        assert_eq!(arena.free_chunks(),1);
        let chunk2 = arena.new_chunk();
        assert_eq!(arena.free_chunks(),0);
        drop(chunk);
        assert_eq!(arena.free_chunks(),1);
        drop(chunk2);
        assert_eq!(arena.free_chunks(),2);

    }
}