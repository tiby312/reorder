//! Simple crate that will reorder a slice based on a slice of indicies without an auxilliary array. See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

#[test]
fn test() {
    let mut v1 = [50usize, 40, 70, 60, 90];
    let mut v2 = [3usize, 0, 4, 1, 2];
    let mut a1 = v1.clone();
    let mut a2 = v2.clone();

    reorder_index(&mut v1, &mut v2);
    reorder_index_aux(&mut a1, &mut a2);
    dbg!(v1, a1);

    for (a, b) in v1.iter().zip(a1.iter()) {
        assert_eq!(a, b);
    }

    for (a, b) in v2.iter().zip(a2.iter()) {
        assert_eq!(a, b);
    }
}

pub fn reorder_index_aux<A: Clone + Default>(arr: &mut [A], index: &mut [usize]) {
    let v = reorder_index_aux_vec(arr, index);
    arr.clone_from_slice(&v);
}

fn reorder_index_aux_vec<A: Clone + Default>(arr: &[A], index: &mut [usize]) -> Vec<A> {
    assert_eq!(arr.len(), index.len());

    let mut res = vec![std::default::Default::default(); arr.len()];

    for (i, (a, b)) in index.iter_mut().zip(arr.iter()).enumerate() {
        res[*a] = b.clone();
        *a = i;
    }
    res
}

pub fn reorder_index<T>(arr: &mut [T], index: &mut [usize]) {
    assert_eq!(arr.len(), index.len());
    // Fix elements one by one
    // n.b.
    //   The above assert and the nature of this loop are likely to eliminate
    //   any bounds checks
    for i in 0..arr.len() {
        let mut target = index[i];
        while i != target {
            index.swap(i, target);
            arr.swap(i, target);
            target = index[i];
        }
    }
}
