use std::{ptr,mem,cmp};
use std::raw::Slice;

#[derive(Show)]
pub struct Chunk {
    mem: *mut u8,
    len: usize,
    cap: usize
}

impl Chunk {
    pub fn new(ptr: *mut u8, cap: usize) -> Chunk {
        Chunk {
            mem: ptr,
            len: 0,
            cap: cap
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        if self.cap == self.len {
            return 0
        }

        let dlen = cmp::min(self.cap-self.len, data.len());

        unsafe { ptr::copy_memory(self.mem.offset(self.len as isize),data[0..dlen].as_ptr(),dlen); }

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

#[cfg(test)]
mod test {
    use std::{mem};
    use std::rt::heap;

    use super::Chunk;

    #[test]
    fn test_base_write_read() {
        let ptr = unsafe { heap::allocate(4, mem::min_align_of::<u8>()) };

        let mut chunk = Chunk::new(ptr,4);
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

        let mut chunk = Chunk::new(ptr,4);
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

        let mut chunk = Chunk::new(ptr,2);
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
}