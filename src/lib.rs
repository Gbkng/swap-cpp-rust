pub fn naive_swap<T: Copy + Default>(a: &mut Vec<T>, b: &mut Vec<T>) {
    assert!(a.len() == b.len());
    let mut buff: T = T::default();
    a.iter_mut().zip(b.iter_mut()).for_each(|(va, vb)| {
        buff = *va;
        *va = *vb;
        *vb = buff;
    });
}

#[test]
fn test_naive_swap(){
    let mut foo = vec![0; 5];
    let mut bar = vec![1; 5];

    std::mem::swap(&mut foo, &mut bar);

    foo.iter().for_each(|v| {assert!(*v == 1);});
    bar.iter().for_each(|v| {assert!(*v == 0);});
}