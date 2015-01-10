use std::cell::RefCell;
use std::rc::Rc;
use std::iter::AdditiveIterator;


use {Arena,ArenaCtl};
use chunk::Chunk;

pub struct NetBuf {
    arena_ctl: Rc<RefCell<ArenaCtl>>,
    chunks: Vec<Chunk>
}

impl NetBuf {
    pub fn new(arena_ctl: Rc<RefCell<ArenaCtl>>) -> NetBuf {
        NetBuf {
            arena_ctl: arena_ctl,
            chunks: vec!()
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        let mut left = data.len();
        let mut pos = 0;

        if self.chunks.len() == 0 {
            self.chunks.push(self.arena_ctl.borrow_mut().new_chunk());
        }

        while left > 0 {
            let mut written = 0;
            {
                let mut chunk = self.chunks.last_mut().expect("chunk does not exist");
                written = chunk.write(data.slice(pos, data.len()));
            }

            if written == 0 {
                self.chunks.push(self.arena_ctl.borrow_mut().new_chunk());
            } else {
                left -= written;
                pos += written;
            }
        }
    }

    pub fn pull(&mut self, size: usize) -> &[u8] {
        b"te"
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().map(|c| c.len()).sum()
    }
}


#[cfg(test)]
mod test {
    use {NetBuf, Arena};

    #[test]
    fn base() {
        let mut arena = Arena::new(4,1);

        let mut buf = arena.new_buf();
        buf.write(b"test");
        assert_eq!(buf.len(), 4);

        // {
        //     let two_bytes = buf.pull(2);
        //     assert_eq!(two_bytes, b"te");
        // }

        // assert_eq!(buf.len(), 2);
    }
}

