#![allow(unstable)]

extern crate test;
extern crate netbuf;
use test::Bencher;
use test::black_box;
use netbuf::Arena;

const TEST_BIG_DATA:&'static [u8] = b"qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,qwertyuiopasdfghjkl;`zxcvbnm,.    qwertyuioasdfghjklzxcvbnm,qwertyuiasdfghjk`zxcvbnm,";
const TEST_SMALL_DATA:&'static [u8] = b"qwertyuiopasdfghjkl";

#[bench]
fn bench_vec_big(b: &mut Bencher) {
    b.iter(|| {
        for _ in range(0,1000) {
            let mut v = vec!();
            for _ in range(0,100) {
                black_box(v.write(TEST_BIG_DATA))
            }
        }
    });
}

#[bench]
fn bench_net_buf_big(b: &mut Bencher) {
    let arena = Arena::new(4096,100);
    b.iter(|| {
        for _ in range(0,1000) {
            let mut buf = arena.new_buf();
            for _ in range(0,100) {
                buf.write(TEST_BIG_DATA);
            }
        }
    });
}


#[bench]
fn bench_vec_small(b: &mut Bencher) {
    b.iter(|| {
        for _ in range(0,1000) {
            let mut v = vec!();
            for _ in range(0,1000) {
                black_box(v.write(TEST_SMALL_DATA))
            }
        }
    });
}

#[bench]
fn bench_net_buf_small(b: &mut Bencher) {
    let arena = Arena::new(4096,100);
    b.iter(|| {
        for _ in range(0,1000) {
            let mut buf = arena.new_buf();
            for _ in range(0,1000) {
                buf.write(TEST_SMALL_DATA);
            }
        }
    });
}