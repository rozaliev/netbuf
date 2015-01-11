use std::{ptr,mem,cmp};
use std::raw::Slice;
use std::cell::RefCell;
use std::rc::Rc;

use arena::{ChunkToken,Arena,Releasable};



pub struct Chunk<T:Releasable> {
    mem: *mut u8,
    len: usize,
    cap: usize,
    token: ChunkToken,
    releasable: T
}

impl<T:Releasable> Chunk<T> {
    pub fn new(ptr: *mut u8, cap: usize, token: ChunkToken, releasable: T) -> Chunk<T> {
        Chunk {
            mem: ptr,
            len: 0,
            cap: cap,
            token: token,
            releasable: releasable
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        if self.cap == self.len {
            return 0
        }

        let dlen = cmp::min(self.cap-self.len, data.len());

        unsafe { ptr::copy_nonoverlapping_memory(self.mem.offset(self.len as isize),data.as_ptr(),dlen); }

        self.len = self.len + dlen;
        dlen
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { mem::transmute(Slice { data: self.mem, len: self.len }) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }
}

#[unsafe_destructor]
impl<T:Releasable> Drop for Chunk<T> {
    fn drop(&mut self) {
        self.releasable.release(self.token.clone());
    }
}

#[cfg(test)]
mod test {
    use std::{mem};
    use std::rt::heap;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Chunk;
    use Arena;
    use arena::{ChunkToken, Releasable};

    #[derive(Clone)]
    struct TestReleaser {
        tokens: Rc<RefCell<Vec<ChunkToken>>>
    }

    impl TestReleaser {
        fn new() -> TestReleaser {
            TestReleaser {
                tokens: Rc::new(RefCell::new(Vec::new()))
            }
        }
    }

    impl Releasable for TestReleaser {
        fn release(&self, token: ChunkToken){
            self.tokens.borrow_mut().push(token);
        }
    }

    #[test]
    fn test_base_write_read() {
        let ptr = unsafe { heap::allocate(4, mem::min_align_of::<u8>()) };

        let mut chunk = Chunk::new(ptr,4,ChunkToken::new(0,0), TestReleaser::new());
        assert_eq!(chunk.cap(),4);
        assert_eq!(chunk.len(),0);

        let written = chunk.write(b"asdf");
        assert_eq!(written, 4);


        assert_eq!(chunk.as_slice(),b"asdf");

        assert_eq!(chunk.len(),4);
    }

    #[test]
    fn test_append() {
        let ptr = unsafe { heap::allocate(4, mem::min_align_of::<u8>()) };

        let mut chunk = Chunk::new(ptr,4,ChunkToken::new(0,0), TestReleaser::new());
        assert_eq!(chunk.cap(),4);
        assert_eq!(chunk.len(),0);

        let written = chunk.write(b"as");
        assert_eq!(written, 2);

        assert_eq!(chunk.as_slice(),b"as");
        assert_eq!(chunk.len(),2);

        let written = chunk.write(b"df");
        assert_eq!(written, 2);

        assert_eq!(chunk.as_slice(),b"asdf");
        assert_eq!(chunk.len(),4);
    }

    #[test]
    fn test_not_enough_space() {
        let ptr = unsafe { heap::allocate(2, mem::min_align_of::<u8>()) };

        let mut chunk = Chunk::new(ptr,2,ChunkToken::new(0,0), TestReleaser::new());
        assert_eq!(chunk.cap(),2);
        assert_eq!(chunk.len(),0);

        let written = chunk.write(b"asdf");
        assert_eq!(written, 2);

        assert_eq!(chunk.as_slice(),b"as");

        assert_eq!(chunk.len(),2);



        let written = chunk.write(b"qwerty");
        assert_eq!(written, 0);

        assert_eq!(chunk.as_slice(),b"as");
        assert_eq!(chunk.len(),2);
    }

    #[test]
    fn test_releases() {
        let ptr = unsafe { heap::allocate(4, mem::min_align_of::<u8>()) };
        let releaser = TestReleaser::new();
        {
            assert_eq!(releaser.tokens.borrow().len(),0);
            let mut chunk = Chunk::new(ptr,2,ChunkToken::new(33,44), releaser.clone());
        }

        assert_eq!(releaser.tokens.borrow().len(),1);
        assert_eq!(releaser.tokens.borrow()[0],ChunkToken::new(33,44));
    }
}