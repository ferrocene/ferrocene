//@ run-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub trait BlockCipher {
    const BLOCK_SIZE: usize;
}

struct FooCipher;
impl BlockCipher for FooCipher {
    const BLOCK_SIZE: usize = 64;
}

struct BarCipher;
impl BlockCipher for BarCipher {
    const BLOCK_SIZE: usize = 32;
}

#[allow(dead_code)]
pub struct Block<C>(C);

pub fn test<C: BlockCipher, const M: usize>()
where
    [u8; M - C::BLOCK_SIZE]: Sized,
{
    let _ = [0; M - C::BLOCK_SIZE];
}

fn main() {
    test::<FooCipher, 128>();
    test::<BarCipher, 64>();
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated items
