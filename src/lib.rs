
//! Simple crate that will reorder a slice based on a slice of indicies without an auxilliary array. See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

extern crate unchecked_index;

pub trait HasIndex{
	fn get(&self)->usize;
	fn set(&mut self,index:usize);
}


#[test]
fn test(){
    let mut v1:Vec<_>=(0usize..10).collect();

    let mut v2:Vec<_>=(0usize..10).rev().collect();
    v2.swap(2,7);


    impl HasIndex for usize{
        fn get(&self)->usize{
            *self
        }
        fn set(&mut self,index:usize){
            *self=index;
        }
    }
    println!("before: \n v1={:?} \n v2={:?}",v1,v2);
    
    reorder(&mut v1,&mut v2);

    println!("after: \n v1={:?} \n v2={:?}",v1,v2);
    panic!("fail");
}


pub fn reorder_no_bounds_checking<'a,A:Copy,B:HasIndex>(arr:&'a mut [A],index:&mut [B]){

    
    assert_eq!(arr.len(),index.len());

    let mut arr=unsafe{unchecked_index::unchecked_index(arr)};


	for i in 0..arr.len(){
		while index[i].get()!=i{
			let old_target_i = index[index[i].get()].get();
			let old_target_e = arr[index[i].get()];

			arr[index[i].get()] = arr[i];
			index[index[i].get()].set(index[i].get());

			index[i].set(old_target_i);
			arr[i] = old_target_e;
		}
	}
}

pub fn reorder<'a,A:Copy,B:HasIndex>(arr:&'a mut [A],index:&mut [B]){

    
    assert_eq!(arr.len(),index.len());

    //let mut arr=unsafe{unchecked_index::unchecked_index(arr)};


    for i in 0..arr.len(){
        while index[i].get()!=i{
            let old_target_i = index[index[i].get()].get();
            let old_target_e = arr[index[i].get()];

            arr[index[i].get()] = arr[i];
            index[index[i].get()].set(index[i].get());

            index[i].set(old_target_i);
            arr[i] = old_target_e;
        }
    }
}
