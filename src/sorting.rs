use rand::Rng;



pub fn quicksort(list: & mut [u32]) {
    if list.len() <= 1 { return () }
    let m = partition(list);
    quicksort(& mut list[..m]);
    quicksort(& mut list[m..]);
}

fn partition(list: & mut [u32]) -> usize {
    let mut rng = rand::thread_rng();
    let last = list.len();
    let p = list[rng.gen_range(0..last)];
    let mut m = 0;
    for i in 0..last {
        if list[i] < p {
            let _ = &list.swap(i, m);
            m += 1;
        }
    }
    m
}