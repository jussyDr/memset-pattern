#![no_main]

use std::num::NonZero;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use memset_pattern::memset_pattern;

#[derive(Arbitrary, Debug)]
struct Input {
    src_len: NonZero<u8>,
    dst_len: u8,
}

fuzz_target!(|input: Input| {
    let src = vec![0; input.src_len.get() as usize];
    let mut dst = vec![0; input.dst_len as usize];
    let mut naive_dst = vec![0; input.dst_len as usize];

    memset_pattern(&mut dst, &src);
    memset_pattern_naive(&mut naive_dst, &src);

    assert_eq!(dst, naive_dst);
});

fn memset_pattern_naive(dst: &mut [u8], src: &[u8]) {
    for i in 0..dst.len() {
        dst[i] = src[i % src.len()];
    }
}
