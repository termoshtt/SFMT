//!  Rust implementation of [SIMD-oriented Fast Mersenne Twister (SFMT)] using [stable SIMD]
//!
//! [SIMD-oriented Fast Mersenne Twister (SFMT)]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/
//! [stable SIMD]: https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md

#![feature(stdsimd)]

extern crate rand;

mod sfmt;

use rand::Rng;
use std::simd::*;

/// State of SFMT
///
/// This struct implements random number generation through `rand::Rng`.
#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    state: [i32x4; sfmt::SFMT_N],
    /// index counter to the 32-bit internal state array
    idx: usize,
}

impl SFMT {
    /// Create a new state from a seed.
    pub fn new(seed: u32) -> Self {
        let mut sfmt = SFMT {
            state: [i32x4::new(0, 0, 0, 0); sfmt::SFMT_N],
            idx: 0,
        };
        sfmt::sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }

    fn pop32(&mut self) -> u32 {
        let val = self.state[self.idx / 4].extract((self.idx % 4) as u32) as u32;
        self.idx += 1;
        val
    }

    fn pop64(&mut self) -> u64 {
        assert!(self.idx % 2 == 0);
        let v: u64x2 = self.state[self.idx / 4].into();
        let idx = (self.idx % 4) / 2;
        self.idx += 2;
        v.extract(idx as u32)
    }

    fn gen_all(&mut self) {
        sfmt::sfmt_gen_rand_all(self);
        self.idx = 0;
    }
}

impl Rng for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.idx >= sfmt::SFMT_N32 {
            self.gen_all();
        }
        self.pop32()
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx >= sfmt::SFMT_N32 {
            self.gen_all();
        }
        self.pop64()
    }
}
