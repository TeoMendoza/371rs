use splits::split_at_mut;

fn main() {
    let mut split = [1i32, 2i32, 3i32, 4i32, 5i32];

    unsafe {    
        let (split_a, split_b) = split_at_mut(&mut split, 2);
        println!("Split A Length: {}, Split B Length {}, Split Total Length {}", split_a.len(), split_b.len(), split.len());
    }
}



