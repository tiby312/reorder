
//! Simple crate that will reorder a slice based on a slice of indicies without an auxilliary array. See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

extern crate unchecked_index;


#[test]
fn test(){
    let mut v1=[50,40,70,60,90];
    let mut v2=[3,0,4,1,2];
    let mut a1=v1.clone();
    let mut a2=v2.clone();
    
    reorder_index(&mut v1,&mut v2);
    reorder_index_aux(&mut a1,&mut a2);

    for (a,b) in v1.iter().zip(a1.iter()){
        assert_eq!(a,b);
    }


    for (a,b) in v2.iter().zip(a2.iter()){
        assert_eq!(a,b);
    }

}






#[inline]
pub fn swap_index(bla:impl ExactSizeIterator<Item=u32>)->Vec<u32>{
    let len=bla.len();
    let mut vec=Vec::with_capacity(len);
    let arr:&mut [u32]=unsafe{std::slice::from_raw_parts_mut(vec.as_mut_ptr(),bla.len())};
    for (i,a) in bla.enumerate(){
        arr[a as usize]=i as u32;
    }

    unsafe{
    vec.set_len(len);
    }
    vec
}



#[inline]
pub fn reorder_index_aux<'a,A:Copy>(arr:&'a mut [A],index:&mut [u32]){
    let mut v=Vec::new();
    v.extend_from_slice(arr);

    for (i,(a,index)) in v.iter().zip(index.iter_mut()).enumerate(){
        arr[*index as usize]=*a;
        *index=i as u32
    }
}


#[inline]
pub fn reorder_index<'a,A:Copy>(arr:&'a mut [A],index:&mut [u32]){

    
    assert_eq!(arr.len(),index.len());

    //let mut index=unsafe{unchecked_index::unchecked_index(index)};


    for i in 0..arr.len(){
        while index[i] as usize!=i{
            let old_target_i = index[index[i] as usize];
            let old_target_e = arr[index[i] as usize];

            arr[index[i] as usize] = arr[i];
            index[index[i] as usize]=index[i];

            index[i]=old_target_i;
            arr[i] = old_target_e;
        }
    }
}

