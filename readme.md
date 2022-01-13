

Simple crate that will reorder a slice based on a slice of indices without an auxiliary array. 

Check it out on [crates.io](https://crates.io/crates/reorder) and [github](https://github.com/tiby312/reorder) and [docs.rs](https://docs.rs/reorder/).

### Example

```rust
fn main() {
    let (mut arr, mut ind, res) = (
        [50, 40, 70, 60, 90, 10],
        [3, 0, 4, 1, 2, 5],
        [40, 60, 90, 50, 70, 10],
    );
    reorder_index(&mut arr, &mut ind);
    assert_eq!(arr, res);
}
```