use bitvec::prelude::*;

use crate::bloom_filter::BloomFilter;

#[derive(Debug)]
pub struct BloomFilterProd {
    bits: BitVec,
    hash_count: usize,
}

impl BloomFilterProd {
    pub fn new(elements: usize, false_probability: f32) -> Self {
        let log2 = 2f32.ln();

        // m = -n * log2(p) / ln(2)
        let bit_count = -((elements as f32 * false_probability.log2()) / log2).ceil() as usize;
        // k = m/n * ln(2)
        let hash_count = (bit_count as f32 / elements as f32 * log2).ceil() as usize;

        Self {
            bits: bitvec![0; bit_count],
            hash_count,
        }
    }

    fn hash(&self, key: &str, seed: usize) -> usize {
        let hash = seahash::hash_seeded(key.as_bytes(), seed as u64, 0, 0, 0) as usize;
        hash % self.bits.len()
    }
}

impl BloomFilter for BloomFilterProd {
    fn insert(&mut self, key: &str) {
        for i in 0..self.hash_count {
            let hash = self.hash(key, i);

            self.bits.set(hash, true)
        }
    }

    fn contains(&self, key: &str) -> bool {
        (0..self.hash_count).all(|i| {
            let hash = self.hash(key, i);
            self.bits[hash]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_zeros() {
        let bl = BloomFilterProd::new(10, 0.01);
        bl.bits.iter().for_each(|bit| assert!(!bit));
    }

    #[test]
    fn test_hash_order_sensitive() {
        let bl = BloomFilterProd::new(10, 0.01);
        let hash1 = bl.hash("mango", 0);
        let hash2 = bl.hash("mango", 1);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_palindrome_sensitive() {
        let bl = BloomFilterProd::new(10, 0.01);
        let hash1 = bl.hash("mango", 0);
        let hash2 = bl.hash("ognam", 0);
        assert_ne!(hash1, hash2);
    }
}
