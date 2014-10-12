bloom-rs
========

Scalable Bloom filters for the [Rust Programming Language](http://rust-lang.org). Functions as a typical bloom filter but does not require the number of elements to be inserted to be specified in advance.

```rust
fn next_capacity_increase(previous_capacity_increase: uint) -> uint {
  previous_capacity_increase * 2
}

let mut filter = bloom::ScalableBloomFilter::new(100, 0.01f64, next_capacity_increase);

filter.insert("Animal");
filter.insert("Vegetable");
filter.insert("Mineral");

if(filter.might_contain("Vegetable")){
  println!("It's very probably been inserted.");
}
```
