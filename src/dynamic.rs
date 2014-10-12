use std::hash::Hash;
use SizedMemory;
use fixed::BloomFilter;

pub struct ScalableBloomFilter {
  items_inserted: uint,
  capacity: uint,
  false_positive_probability: f64,
  current_filter_max_items: uint,
  max_items_grower: fn(uint) -> uint,
  filters: Vec<BloomFilter>
}

impl ScalableBloomFilter {
  pub fn new(starting_max_items: uint, false_positive_probability: f64, max_items_grower: fn(uint) -> uint) -> ScalableBloomFilter {
    let mut bloom_filters : Vec<BloomFilter> = Vec::with_capacity(3);
    bloom_filters.push(BloomFilter::new(starting_max_items, false_positive_probability));
    ScalableBloomFilter {
      items_inserted: 0,
      capacity: starting_max_items,
      false_positive_probability: false_positive_probability,
      current_filter_max_items: starting_max_items,
      max_items_grower: max_items_grower,
      filters: bloom_filters
    }
  }

  pub fn capacity(&self) -> uint {
    self.capacity
  }

  pub fn number_inserted(&self) -> uint {
    self.items_inserted
  }

  pub fn number_of_filters(&self) -> uint {
    self.filters.len()
  }

  pub fn might_contain<T>(&self, key: T) -> bool where T: Hash {
    self.filters.iter().any(|filter| filter.might_contain(&key))
  }
  
  pub fn insert<T>(&mut self, key: T) -> bool where T: Hash {
    if self.items_inserted >= self.capacity {
      self.grow();
    }

    let already_contains_key : bool;
    let num_filters = self.filters.len();
    {
      let mut all_filters_but_last = self.filters.iter().take(num_filters - 1);
      already_contains_key = all_filters_but_last.any(|filter| filter.might_contain(&key));
    }

    if already_contains_key {
      return true;
    }

    let last_filter : &mut BloomFilter = self.filters.get_mut(num_filters - 1);
    last_filter.insert(key);
    self.items_inserted += 1;
    false
  }

  fn grow(&mut self) {
      debug!("Growing the filter set from {} to {}", self.filters.len(), self.filters.len()+1);
      let next_max_items = (self.max_items_grower)(self.current_filter_max_items);
      let filter = BloomFilter::new(next_max_items, self.false_positive_probability);
      self.filters.push(filter);
      self.current_filter_max_items = next_max_items;
      self.capacity += next_max_items;
  }
}

impl SizedMemory for ScalableBloomFilter {
  fn bytes_on_heap(&self) -> u64 {
    self.filters
      .iter()
      .map(|filter| filter.bytes_on_heap())
      .inspect(|num_bytes| println!("subfilter -> {} bytes", num_bytes))
      .fold(0, |a, b| a+b)
  }
}
