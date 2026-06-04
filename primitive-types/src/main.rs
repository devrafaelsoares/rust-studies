use std::mem;

fn main() {

// tuples();
arrays();


}



fn arrays()
{
    let xs: [i32; 500] = [0; 500];

    println!("Array: {:?}", xs);
    println!("First element: {}", xs[0]);
    println!("Last element: {}", xs[499]);

    println!("Array length: {}", xs.len());
    println!("Array occupies {} bytes.", mem::size_of_val(&xs))
}


fn tuples() 
{

    let numbers = (1u64,2u64,3u64,4u64,5u64,6u64,7u64,8u64,9u64,0u64);

    println!("Numbers: {:?}", numbers);
    println!("First number: {}", numbers.0);
    println!("Last number: {}", numbers.9);

    let (first, second, third, fourth, fifth, sixth, seventh,eighth,ninth, tenth) = numbers;

    println!("First number: {}", first);
    println!("Second number: {}", second);
    println!("Third number: {}", third);
    println!("Fourth number: {}", fourth);
    println!("Fifth number: {}", fifth);
    println!("Sixth number: {}", sixth);
    println!("Seventh number: {}", seventh);
    println!("Eighth number: {}", eighth);
    println!("Ninth number: {}", ninth);
    println!("Tenth number: {}", tenth);
}
