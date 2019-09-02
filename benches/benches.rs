 #![feature(test)]
extern crate reorder;
extern crate rand;
extern crate test;
use test::*;


#[derive(Copy,Clone)]
struct Bag{
    _a:[usize;16],
}

fn random_perm(num:usize)->(Vec<Bag>,Vec<u32>){
    use rand::{Rng, SeedableRng, StdRng};

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::seed_from_u64(2432);

    let mut vec:Vec<_>=(0usize..num).map(|i|Bag{_a:[i;16]}).collect();
    rng.shuffle(&mut vec);

    let mut vec2:Vec<_>=(0u32..num as u32).collect();
    rng.shuffle(&mut vec2);

    (vec,vec2)
}


#[bench]
fn bench_reorder_index(b:&mut Bencher){
    b.iter(||{
        let (mut a,mut b)=random_perm(500_000);
        reorder::reorder_index(&mut a,&mut b);
        black_box((a,b));
    });
}

#[bench]
fn bench_reorder_index_aux(b:&mut Bencher){
    b.iter(||{
        let (mut a,mut b)=random_perm(500_000);
        reorder::reorder_index_aux(&mut a,&mut b);
        black_box((a,b));
    });
}

