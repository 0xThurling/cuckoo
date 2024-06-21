fn lookup(first_arr: &[i32; 5], second_arr: &[i32; 5], key: i32) -> bool {
    let mut contains_key: bool = false;

    for i in 0..5 {
        if first_arr[i] == key || second_arr[i] == key {
            contains_key = true;
        }
    }

    contains_key
}

fn main() {
    let first_cuckoo_arr: [i32; 5] = [0; 5];
    let second_cuckoo_arr: [i32; 5] = [0; 5];

    if lookup(&first_cuckoo_arr, &second_cuckoo_arr, 0) {
        return;
    }
}
