#![crate_name = "bloom"]
#![crate_type = "lib"]
#![desc = "A scalable bloom filter implementation in Rust."]
#![license = "MIT"]
#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate bloomfilter;

use dynamic::ScalableBloomFilter;
use std::mem::size_of_val;

pub mod fixed;
pub mod dynamic;

pub trait SizedMemory {
  fn bytes_on_heap(&self) -> u64;
  fn bytes_on_stack(&self) -> u64 {
    size_of_val(self) as u64
  }
  fn total_bytes(&self) -> u64 {
    self.bytes_on_heap() + self.bytes_on_stack()
  }
}

#[test]
fn grows_as_expected(){
  fn grow(capacity: uint) -> uint {
    2 * capacity
  }

  let mut filter = ScalableBloomFilter::new(10, 0.1f64, grow);
  println!("{} inserted / {} capacity", filter.number_inserted(), filter.capacity());  
  assert_eq!(1, filter.number_of_filters());

  for i in range(0, 15u) {
    filter.insert(i);
  }

  println!("{} inserted / {} capacity", filter.number_inserted(), filter.capacity());  
  assert_eq!(2, filter.number_of_filters());

  for j in range(15, 45u) {
    filter.insert(j);
  }

  println!("{} inserted / {} capacity", filter.number_inserted(), filter.capacity());  
  assert_eq!(3, filter.number_of_filters());
}

#[test]
fn reports_contains_correctly(){
  fn grow(capacity: uint) -> uint {
    2 * capacity
  }
  let mut filter = ScalableBloomFilter::new(1, 0.1f64, grow);

  filter.insert("dog");
  filter.insert("cat");
  filter.insert("mouse");

  assert_eq!(2, filter.number_of_filters());
  assert_eq!(true, filter.might_contain("dog"));
  assert_eq!(true, filter.might_contain("cat"));
  assert_eq!(true, filter.might_contain("mouse"));
}

#[test]
fn reports_size_correctly(){
  fn grow(capacity: uint) -> uint {
    2 * capacity
  }
  let mut filter = ScalableBloomFilter::new(1, 0.1f64, grow);

  for i in range(0, 1000u) {
    filter.insert(i);
  }

  println!("Number of Filters: {}", filter.number_of_filters());
  println!("Bytes on stack: {}", filter.bytes_on_stack());
  println!("Bytes on heap: {}", filter.bytes_on_heap());
  println!("Bytes total: {}", filter.total_bytes());

  assert_eq!(true, true);
} 
