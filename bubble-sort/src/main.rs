fn main() {
    let mut arr: [i32; 5] = [9, 4, 2, 7, 1];
    let mut temp: i32;

    for i1 in 0..arr.len() {
        for i2 in i1+1..arr.len() {
            if arr[i1] > arr[i2] {
                temp = arr[i1];
                arr[i1] = arr[i2];
                arr[i2] = temp;    
            }
        }
    }

    println!("{:?}", arr)
}