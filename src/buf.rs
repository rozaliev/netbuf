use std::cell::RefCell;
use std::rc::Rc;
use std::iter::AdditiveIterator;
use std::collections::RingBuf;
use std::io;

use {Arena, NetWriter, PartialResult};
use chunk::Chunk;

pub struct NetBuf {
    arena: Arena,
    chunks: RingBuf<Chunk<Arena>>,
    pos: usize
}

impl NetBuf {
    pub fn new(arena: Arena) -> NetBuf {
        NetBuf {
            arena: arena,
            chunks: RingBuf::new(),
            pos: 0
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        let mut left = data.len();
        let mut pos = 0;

        if self.chunks.len() == 0 {
            self.chunks.push_back(self.arena.new_chunk());
        }

        while left > 0 {
            let mut written = 0;
            {
                let mut chunk = self.chunks.back_mut().expect("chunk does not exist");
                written = chunk.write(data.slice(pos, data.len()));
            }

            if written == 0 {
                self.chunks.push_back(self.arena.new_chunk());
            } else {
                left -= written;
                pos += written;
            }
        }
    }

    pub fn write_to<T:NetWriter>(&mut self, w: &mut T) -> io::IoResult<()> {
        loop {
            match w.write(self.chunks[0].as_slice().slice(self.pos,self.chunks[0].len())) {
                Ok(written) => {
                    self.pos = 0;
                    self.chunks.pop_front();
                    if self.chunks.len() == 0 {
                        return Ok(())
                    }
                },
                Err(PartialResult(written,err)) => {
                    self.pos += written;
                    return Err(err)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().map(|c| c.len()).sum() - self.pos
    }
}


#[cfg(test)]
mod test {
    use {NetBuf, Arena, NetWriter, PartialResult};
    use std::io;
    use std::cmp;

    impl NetWriter for Vec<u8> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, PartialResult> {
            let cap = self.capacity();
            let slen = self.len();

            (self as &mut Writer).write(buf.slice(0, cmp::min(cap-slen, buf.len())));

            if buf.len()+slen > cap {
                let err = io::IoError {
                    kind: io::IoErrorKind::OtherIoError,
                    desc: "some error",
                    detail: None
                };
                return Err(PartialResult(cap-slen, err))
            }

            return Ok(buf.len())
        }
    }

    #[test]
    fn test_write_write_to() {
        let mut arena = Arena::new(4,1);

        let mut buf = arena.new_buf();
        buf.write(b"test");
        assert_eq!(buf.len(), 4);

        let mut res = Vec::with_capacity(4);
        match buf.write_to(&mut res) {
            Ok(_) => {},
            Err(err) => { panic!("write_to failed: {}", err) }
        }

        assert_eq!(res.as_slice(), b"test");
    }

    #[test]
    fn test_chunks() {
        let mut arena = Arena::new(2,5);

        let mut buf = arena.new_buf();
        buf.write(b"asdf");
        assert_eq!(buf.len(), 4);

        let mut res = Vec::with_capacity(4);
        match buf.write_to(&mut res) {
            Ok(_) => {},
            Err(err) => { panic!("write_to failed: {}", err) }
        }

        assert_eq!(res.as_slice(), b"asdf");

        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_pos() {
        let mut arena = Arena::new(2,5);

        let mut buf = arena.new_buf();
        buf.write(b"asdf");
        assert_eq!(buf.len(), 4);

        ////
        let mut res = Vec::with_capacity(1);
        match buf.write_to(&mut res) {
            Ok(_) => { panic!("write_to is expected to fail, res: {:?}, full str: {:?}",res, b"asdf")},
            Err(err) => { }
        }

        assert_eq!(res.as_slice(), b"a");

        assert_eq!(buf.len(), 3);

        ////
        let mut second_res = Vec::with_capacity(2);
        match buf.write_to(&mut second_res) {
            Ok(_) => { panic!("write_to is expected to fail, second_res: {:?}, full str: {:?}", second_res, b"asdf")},
            Err(err) => { }
        }

        assert_eq!(second_res.as_slice(), b"sd");
        assert_eq!(buf.len(), 1);

        ////
        let mut last_res = Vec::with_capacity(2);
        match buf.write_to(&mut last_res) {
            Ok(_) => {},
            Err(err) => { panic!("write_to failed: {}", err) }
        }

        assert_eq!(last_res.as_slice(), b"f");
        assert_eq!(buf.len(), 0);


    }
}

