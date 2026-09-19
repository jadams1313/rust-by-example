fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut array = [5, 4, 3, 2, 1];
    bubble_sort(&mut array);
    println!("{:?}", array); // [1, 2, 3, 4, 5]
}
