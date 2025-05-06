fn main() {
    const DEFAULT_VEC_SIZE: usize = 1_000_000;
    const N_SWAPS: usize = 10_000 - 1; // odd to observe swap after final step

    let args: Vec<String> = std::env::args().collect();

    let vec_size: usize = if args.len() >= 2 {
        args[1].parse().unwrap()
    } else {
        DEFAULT_VEC_SIZE
    };

    println!("vec size: {}\nnswaps: {}", vec_size, N_SWAPS);

    let mut foo = vec![0; vec_size];
    let mut bar = vec![1; vec_size];

    for _ in 0..N_SWAPS {
        std::mem::swap(std::hint::black_box(&mut foo), std::hint::black_box(&mut bar));
    }

    assert!(*foo.first().unwrap() == 1);
    assert!(*foo.last().unwrap() == 1);

    assert!(*bar.first().unwrap() == 0);
    assert!(*bar.last().unwrap() == 0);
}
