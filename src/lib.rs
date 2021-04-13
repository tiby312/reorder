//! Simple crate that will reorder a slice based on a slice of indicies without an auxilliary array. See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

#[test]
fn test() {
    let mut v1 = [50usize, 40, 70, 60, 90];
    let mut v2 = [3usize, 0, 4, 1, 2];
    let mut a1 = v1.clone();
    let mut a2 = v2.clone();

    reorder_index(&mut v1, &mut v2);
    reorder_index_aux(&mut a1, &mut a2);

    let mut bla = Vec::new();
    bla.extend_from_slice(&v1);
    let a3: Vec<usize> = reorder_index_aux_vec(bla, &mut a2);

    for ((a, b), c) in v1.iter().zip(a1.iter()).zip(a3.iter()) {
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    for (a, b) in v2.iter().zip(a2.iter()) {
        assert_eq!(a, b);
    }
}

///
/// # Safety
///
/// Unsafe since the size of the size hint of the iterator is not
/// guarenteed to be equal to the number of items produced by
/// the iterator.
///
/// The user must also guarentee that the provided iterator is a permutation
/// of the indicies.
///
#[inline]
pub unsafe fn swap_index(bla: impl ExactSizeIterator<Item = usize>) -> Vec<usize> {
    let len = bla.len();
    let mut vec = Vec::with_capacity(len);
    let arr: &mut [usize] = std::slice::from_raw_parts_mut(vec.as_mut_ptr(), bla.len());
    for (i, a) in bla.enumerate() {
        arr[a] = i;
    }

    vec.set_len(len);

    vec
}

#[inline]
pub fn reorder_index_aux<A>(arr: &mut [A], index: &mut [usize]) {
    let mut v = Vec::with_capacity(arr.len());
    unsafe {
        std::ptr::copy_nonoverlapping(arr.as_ptr(), v.as_mut_ptr(), arr.len());
        v.set_len(arr.len());
    }

    for (i, (a, index)) in v.drain(..).zip(index.iter_mut()).enumerate() {
        let res = core::mem::replace(&mut arr[*index], a);

        *index = i;
        core::mem::forget(res);
    }
}

#[inline]
pub fn reorder_index_aux_vec<A>(mut arr: Vec<A>, index: &mut [usize]) -> Vec<A> {
    let mut v = Vec::with_capacity(arr.len());
    unsafe { v.set_len(arr.len()) };
    let vv = unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr(), v.len()) };

    for (i, (a, index)) in arr.drain(..).zip(index.iter_mut()).enumerate() {
        vv[*index] = a;
        *index = i
    }

    v
}

use uninit::prelude::*;

pub fn reorder_index<A>(arr: &mut [A], indices: &mut [usize]) {
    assert_eq!(arr.len(), indices.len());

    let arr: &mut [MaybeUninit<_>] = unsafe { arr.manually_drop_mut().as_out().as_mut_uninit() };
    for i in 0..arr.len() {
        while indices[i] != i {
            let old_target_i = indices[indices[i]];

            //let old_target_e = arr[index[i]];
            let mut old_target_e = std::mem::MaybeUninit::<A>::uninit();
            unsafe {
                old_target_e
                    .as_mut_ptr()
                    .write(arr[indices[i]].as_ptr().read());
            }

            // arr[index[i]] = arr[i];
            // Since the object arr[index[i]] is aliased to old_target_e, we cannot produce
            // mutable reference to it nor assign to it.
            unsafe {
                arr[indices[i]].as_mut_ptr().write(arr[i].as_ptr().read());
            }
            // Now that old_target_e is not longer aliased, we can call assume_init
            let old_target_e = unsafe { old_target_e.assume_init() };

            indices[indices[i]] = indices[i];

            indices[i] = old_target_i;

            //arr[i] = old_target_e;
            // arr[i] is aliased to arr[index[i]
            arr[i].as_out().write(old_target_e);
        }
    }
}
