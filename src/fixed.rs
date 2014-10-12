use bloomfilter::Bloom;
use std::hash::Hash;
use SizedMemory;

pub struct BloomFilter {
  filter: Bloom
}

impl BloomFilter {
  pub fn new(maximum_items: uint, false_positive_probability: f64) -> BloomFilter {
    BloomFilter{
      filter: Bloom::new_for_fp_rate(maximum_items, false_positive_probability)
    }
  }

  pub fn might_contain<T>(&self, key: T) -> bool where T: Hash {
    self.filter.check(key)
  }
  
  pub fn insert<T>(&mut self, key: T) -> bool where T: Hash {
    self.filter.check_and_set(key)
  }
}

impl SizedMemory for Bloom {
  fn bytes_on_heap(&self) -> u64 {
    self.number_of_bits() / 8
  }
}

impl SizedMemory for BloomFilter {
  fn bytes_on_heap(&self) -> u64 {
    self.filter.bytes_on_heap() 
  }
}
