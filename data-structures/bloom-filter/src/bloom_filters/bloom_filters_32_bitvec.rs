use bitvec::prelude::*;

use crate::bloom_filter::BloomFilter;

pub struct BloomFilter32BitVec {
    bits: BitArr!(for 32, in u32, Msb0),
}

impl Default for BloomFilter32BitVec {
    fn default() -> Self {
        Self {
            bits: BitArray::ZERO,
        }
    }
}

impl BloomFilter32BitVec {
    fn additive_hasher(&self, key: &str) -> usize {
        key.chars()
            .fold(0, |acc, ch| -> usize { (acc + (ch as usize % 32)) % 32 })
    }

    fn place_value_hasher(&self, key: &str) -> usize {
        key.chars()
            .enumerate()
            .fold(0, |acc, (index, ch)| -> usize {
                let char_value = ((ch as usize) % 32) * 26usize.pow(index as u32 + 2);
                (acc + (char_value % 32)) % 32
            })
    }
}

impl std::fmt::Debug for BloomFilter32BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits: String = self
            .bits
            .iter()
            .by_vals()
            .map(|x| if x { "1 " } else { "0 " })
            .collect();
        write!(f, "{:?}", bits)
    }
}

impl BloomFilter for BloomFilter32BitVec {
    fn insert(&mut self, key: &str) {
        let hash_a = self.additive_hasher(key);
        let hash_b = self.place_value_hasher(key);

        self.bits.set(hash_a, true);
        self.bits.set(hash_b, true);
    }

    fn contains(&self, key: &str) -> bool {
        let hash_a = self.additive_hasher(key);
        let hash_b = self.place_value_hasher(key);

        self.bits[hash_a] && self.bits[hash_b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_zeros() {
        let bl = BloomFilter32BitVec::default();

        bl.bits.iter().for_each(|bit| assert!(!bit));
    }

    #[test]
    fn test_addititve_hasher_empty_string() {
        let bl = BloomFilter32BitVec::default();
        assert_eq!(bl.additive_hasher(""), 0);
    }
}
