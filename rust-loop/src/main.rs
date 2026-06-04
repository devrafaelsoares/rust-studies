fn main() {

    // Creating a for loop in Rust

    println!("======== For Loop ========");


    for i_for in 0..10 {
        println!("Value: {i_for}");
    }

    // Creating a while loop in Rust

    let mut i_while = 0;

    println!("======== While Loop ========");

    while i_while <= 10 {
        println!("Value: {i_while}");
        i_while += 1;
    }

    // Creating a "loop" loop in Rust

    println!("======== 'Loop' Loop ========");


    let mut i_loop = 0;

    loop {

        if i_loop >= 10 {
            break
        }

        println!("Value: {i_loop}");
        i_loop += 1;
    }

}

