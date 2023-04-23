// f"{}"
// `${}`

// u32 int
// u64 long
// u128 long long
// long a = SOME_BIG_NUMBER;

fn main() {
    let x = 5;

    {
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}