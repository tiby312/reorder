
//! Simple crate that will reorder a slice based on a slice of indicies without an auxilliary array. See https://www.geeksforgeeks.org/reorder-a-array-according-to-given-indexes/

extern crate unchecked_index;


#[test]
fn test(){
    let mut v1=[50,40,70,60,90];
    let mut v2=[3,0,4,1,2];
    let mut a1=v1.clone();
    let mut a2=v2.clone();
    let mut b1=v1.clone();
    let mut b2=v2.clone();
    
    
    reorder_index(&mut v1,&mut v2);
    reorder_index_aux(&mut a1,&mut a2);

    let mut bla=Vec::new();
    bla.extend_from_slice(&v1);
    let a3:Vec<usize>=reorder_index_aux_vec(bla,&mut a2);


    for ((a,b),c) in v1.iter().zip(a1.iter()).zip(a3.iter()){
        assert_eq!(a,b);
        assert_eq!(b,c);
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
pub fn reorder_index_aux<'a,A>(arr:&mut [A],index:&mut [u32]){
    
    
    let mut v=Vec::with_capacity(arr.len());
    unsafe{
        std::ptr::copy_nonoverlapping(arr.as_ptr(), v.as_mut_ptr(), arr.len());
        v.set_len(arr.len());
    }

    for (i,(a,index)) in v.drain(..).zip(index.iter_mut()).enumerate(){
        let res=core::mem::replace(&mut arr[*index as usize],a);

        //arr[*index as usize]=a;
        *index=i as u32;
        core::mem::forget(res);
    }
    
}




#[inline]
pub fn reorder_index_aux_vec<'a,A>(mut arr:Vec<A>,index:&mut [u32])->Vec<A>{
    
    
    let mut v=Vec::with_capacity(arr.len());
    unsafe{v.set_len(arr.len())};
    let vv=unsafe{std::slice::from_raw_parts_mut(v.as_mut_ptr(),v.len())};


    for (i,(a,index)) in arr.drain(..).zip(index.iter_mut()).enumerate(){
        vv[*index as usize]=a;
        *index=i as u32
    }

    v
    
}


#[inline]
pub fn reorder_index<'a,A>(arr:&'a mut [A],index:&mut [u32]){

    
    assert_eq!(arr.len(),index.len());


    

    for i in 0..arr.len(){
        while index[i] as usize!=i{
            let old_target_i = index[index[i] as usize];
            

            let mut old_target_e:A=unsafe{std::mem::MaybeUninit::<A>::zeroed().assume_init()};
            core::mem::swap(&mut old_target_e,&mut arr[index[i] as usize]);
            //let old_target_e = arr[index[i] as usize];

            //arr[index[i] as usize] = arr[i];
            let kk:&mut _=unsafe{&mut *(&mut arr[i] as *mut _)};
            core::mem::swap(&mut arr[index[i] as usize],kk);

            index[index[i] as usize]=index[i];

            index[i]=old_target_i;

            //arr[i] = old_target_e;
            core::mem::swap(&mut arr[i],&mut old_target_e);

            core::mem::forget(old_target_e);
        }
    }
}

