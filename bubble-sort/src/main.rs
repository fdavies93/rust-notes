fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut output = arr.clone();

    let mut temp: i32;

    for i1 in 0..output.len() {
        for i2 in i1+1..output.len() {
            if output[i1] > output[i2] {
                temp = output[i1];
                output[i1] = output[i2];
                output[i2] = temp;    
            }
        }
    }
    return output;
}

fn main() {
    let arr = vec![9,4,2,7,1];

    let sorted = bubble_sort(arr);

    println!("{:?}", sorted)
}