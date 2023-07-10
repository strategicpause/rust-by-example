use bit_vec::BitVec;
use core::hash::Hash;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::f64;
use std::hash::{BuildHasher, Hasher};
use std::marker::PhantomData;


// The `?Sized` trait is used to indicate to the compiler that the type does not
// have a size known at compile-time.
pub struct BloomFilter<T: ?Sized> {
    // Determines the number of bits to set in the BitVec. This is also used to determine how many
    // hash functions to use when setting bits in the bit vector.
    k_num: usize,
    // The size pf the BitVec which is a function of the number of elements that we want to store
    // for some false positive probability.
    size: usize,
    bit_vec: BitVec,
    // Only two hash functions are used in the bloom filter as it is demonstrated by Kirsch &
    // Mitzenmacher that two hash functions can be used to simulate additional hash functions
    // in the form: g(x) = h_1(x) + i * h_2(x).
    hashers: [DefaultHasher; 2],
    // PhantomData refers to a zero-sized type to tell the compiler that BloomFilter
    // acts as though it stores a value of type `T`, when it doesn't really.
    _phantom: PhantomData<T>,
}


impl<T: ?Sized> BloomFilter<T> {
    /// new returns an initialized Bloom filter.
    /// # Arguments
    /// * `num_elements` - Provides an estimation of the maximum number of elements to store.
    /// * `probability` - Indicates the rate of false positives. Can be a value between (0.0, 1.0)
    pub fn new(num_elements: usize, probability: f64) -> BloomFilter<T> {
        let bitmap_size = Self::bitmap_size(num_elements, probability);
        Self {
            k_num: Self::k_num(probability),
            size: bitmap_size,
            // Create a new BitVec of a given size with each value set to 0 (false).
            bit_vec: BitVec::from_elem(bitmap_size, false),
            hashers: [
                RandomState::new().build_hasher(),
                RandomState::new().build_hasher(),
            ],
            _phantom: PhantomData,
        }
    }

    /// bitmap_size will compute a size for the bitmap vector to store `n` items with a
    /// probability `p` of false positives.
    ///
    /// n * (ln(p) / -8 * ln(2)^2)
    fn bitmap_size(n: usize, p: f64) -> usize {
        assert!(n > 0);
        assert!(p > 0.0 && p < 1.0);

        let ln2_squared = f64::consts::LN_2 * f64::consts::LN_2;
        ((n as f64) * f64::ln(p) / -ln2_squared).ceil() as usize
    }

    // k_num will calculate the number of hashing functions to use to achieve the given
    // false positive probability.
    fn k_num(p: f64) -> usize {
        (-p.ln() / f64::consts::LN_2).ceil() as usize
    }

    /// Adds a value to the bloom filter. The value must implement the `Hash` trait.
    pub fn add(&mut self, value: &T) where T: Hash {
        let (h1, h2) = self.hash_kernel(value);
        for k in 0..self.k_num {
            let index = self.bit_index(k, h1, h2);
            self.bit_vec.set(index, true);
        }
    }

    fn hash_kernel(&self, item: &T) -> (usize, usize) where T: Hash {
        let (h1, h2) = (&mut self.hashers[0].clone(), &mut self.hashers[1].clone());

        item.hash(h1);
        item.hash(h2);

        (h1.finish() as usize, h2.finish() as usize)
    }

    // Get the bit index from the kth hash function. For example, the kth hash function might
    // like the following:
    // 0 -> h1
    // 1 -> h1 + h2
    // k -> h1 + n * h2
    fn bit_index(&self, k: usize, h1: usize, h2: usize) -> usize {
        h1.wrapping_add(k).wrapping_mul(h2) % self.size
    }

    /// Returns true if the given element might be in the BloomFilter. Always returns false if the
    /// element is not in the BloomFilter.
    pub fn contains(&self, value: &T) -> bool where T: Hash {
        let (h1, h2) = self.hash_kernel(value);
        for k in 0..self.k_num {
            let index = self.bit_index(k, h1, h2);
            if !self.bit_vec.get(index).unwrap() {
                return false;
            }
        }
        true
    }
}
