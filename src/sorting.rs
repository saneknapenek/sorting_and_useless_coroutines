use rand::Rng;



pub fn quicksort(list: & mut [u32], first_pivot_i: Option<usize>) {
    if list.len() <= 1 { return () }
    let piv = match first_pivot_i {
        Some(value) => value,
        None => {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..list.len())
        }
    };
    let m = partition(list, piv);
    quicksort(& mut list[..m], None);
    quicksort(& mut list[m..], None);
}

fn partition(list: & mut [u32], piv:usize) -> usize {
    let last = list.len();
    let p = list[piv];
    let mut m = 0;
    for i in 0..last {
        if list[i] < p {
            let _ = &list.swap(i, m);
            m += 1;
        }
    }
    m
}