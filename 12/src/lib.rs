pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {

    let ptr = values.as_mut_ptr();
    let mid = std::cmp::min(values.len(), mid);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), values.len() - mid),
        )
    }
}