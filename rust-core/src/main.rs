fn main() {
    let s1 = String::from("Clean Architecture");

    let s2 = &s1;
    let s3 = &s2;
    let s4 = s1.clone();
    let s5 = &s4;
    println!("The original string is: {s1}, and the borrowed string is: {s2}, and the borrowed string is: {s3}, and the cloned string is: {s4}, and the borrowed string is: {s5}");
}