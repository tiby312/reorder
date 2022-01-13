

Simple crate that will reorder a slice based on a slice of indices without an auxiliary array. 

Check it out on [crates.io](https://crates.io/crates/reorder) and [github](https://github.com/tiby312/reorder) and [docs.rs](https://docs.rs/reorder/).

### Example

```rust
#[test]
fn test() {
    let (mut arr, mut ind, res) = (
        [50usize, 40, 70, 60, 90],
        [3usize, 0, 4, 1, 2],
        [40, 60, 90, 50, 70],
    );
    reorder_index(&mut arr, &mut ind);
    assert_eq!(arr, res);
}
```