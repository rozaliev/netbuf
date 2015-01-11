extern crate netbuf;

use netbuf::Arena;

const TEST_DATA:&'static [u8] = b"qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,";

fn main() {
    let arena = Arena::new(4096,100);

    loop {
        let mut buf = arena.new_buf();
        for i in range(0,100) {
            buf.write(TEST_DATA);
        }
    }
}