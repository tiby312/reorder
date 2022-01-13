//! Simple crate that will reorder a slice based on a slice of indices without an auxiliary array.
//! See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

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

#[test]
fn test_aux() {
    let (mut arr, mut ind, res) = (
        [50usize, 40, 70, 60, 90],
        [3usize, 0, 4, 1, 2],
        [40, 60, 90, 50, 70],
    );
    reorder_index_aux(&mut arr, &mut ind);
    assert_eq!(arr, res);
}
pub fn reorder_index_aux<A: Clone + Default>(arr: &mut [A], index: &[usize]) {
    let v = reorder_index_aux_vec(arr, index);
    arr.clone_from_slice(&v);
}

fn reorder_index_aux_vec<A: Clone + Default>(arr: &[A], index: &[usize]) -> Vec<A> {
    assert_eq!(arr.len(), index.len());

    let mut res = vec![std::default::Default::default(); arr.len()];

    for (a, b) in index.iter().zip(arr.iter()) {
        res[*a] = b.clone();
    }
    res
}

pub fn reorder_index<T>(arr: &mut [T], index: &mut [usize]) {
    assert_eq!(arr.len(), index.len());
    for i in 0..arr.len() {
        let mut target = index[i];
        while i != target {
            index.swap(i, target);
            arr.swap(i, target);
            target = index[i];
        }
    }
}
