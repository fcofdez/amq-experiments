extern crate bit_vec;
extern crate fasthash;

use std::hash::{Hash, Hasher};
use fasthash::{murmur, MurmurHasher};
use bit_vec::BitVec;
use std::marker::PhantomData;


pub struct Bloom<T> {
    bitmap: BitVec,
    bitmap_bits: u64,
    k_num: u32,
    hashers: [MurmurHasher; 2],
    _phantom: PhantomData<T>,
}

impl<T> Bloom<T> {
    pub fn new(bitmap_size: usize, items_count: usize) -> Self {
        let bitmap_bits = (bitmap_size as u64) * 8u64;
        let mut r: MurmurHasher = Default::default();
        let mut s: MurmurHasher = Default::default();
        let hashers = [r, s];
        // let k_num = Self::optimal_k_num(bitmap_bits, items_count);
        let k_num: u32 = 2;
        let bitmap = BitVec::from_elem(bitmap_bits as usize, false);
        Self {
            bitmap: bitmap,
            bitmap_bits: bitmap_bits,
            k_num: k_num,
            hashers: hashers,
            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, item: &T)
        where T: Hash
    {
        let mut hashes = [0u64, 0u64];
        for k_i in 0..self.k_num {
            let hasher = &mut self.hashers[k_i as usize].clone();
            item.hash(hasher);
            let hash = hasher.finish();
            let offset = (hash % self.bitmap_bits) as usize;
            self.bitmap.set(offset, true);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
