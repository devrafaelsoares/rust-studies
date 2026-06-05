fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

fn is_even(n: i32) -> bool {

    n % 2 == 0
}

fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    a.max(b).max(c)
}

fn max_of(numbers: &[i32]) -> i32 {

    let mut max_number = numbers[0];

    for number in numbers {
       
        if *number >= max_number {
            max_number = *number
        }
    }

    max_number
}

fn first_word(s: &str) -> &str {
    
    const EMPTY_CHAR: char = ' ';
    let mut count = 0;
    let s = s.trim();
   
    for char in s.chars() {

        if char == EMPTY_CHAR {
            return &s[0..count];
        }
      
      count += 1;
    }
    
    return s
}



fn count_vowels(s: &str) -> usize {
    const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
   
    s.chars()
        .flat_map(|c| c.to_uppercase())
        .map(|c| match c {
            'Á' | 'À' | 'Ã' | 'Â' | 'Ä' => 'A',
            'É' | 'È' | 'Ê' | 'Ë' => 'E',
            'Í' | 'Ì' | 'Î' | 'Ï' => 'I',
            'Ó' | 'Ò' | 'Õ' | 'Ô' | 'Ö' => 'O',
            'Ú' | 'Ù' | 'Û' | 'Ü' => 'U',
            _ => c,
        })
        .filter(|c| VOWELS.contains(c))
        .count()

}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn custom_reverse_string(s: &str) -> String {
    let mut letters: Vec<char> = s.chars().collect();

    if letters.is_empty() { return String::from(""); }

    let mut left = 0;
    let mut right = letters.len() - 1;

    while left < right {
        let temp = letters[left];
        letters[left] = letters[right];
        letters[right] = temp;

        left += 1;
        right -= 1;
    }

    letters.into_iter().collect()
}


fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }

    fibonacci(n - 1) + fibonacci(n - 2) 
}


fn factorial(n: u64) -> u64 {

    if n <= 1 { return 1; }

    n * factorial(n - 1) 

}

fn main() {
    println!("=== 🧮 Testing Basic Functions ===");
    
    let sum = add(10, 5);
    println!("add(10, 5) => {sum}");
    
    let greeting = greet("Rafael");
    println!("greet(\"Rafael\") => {greeting}");
    
    let is_even_result = is_even(42);
    let is_odd_result = is_even(7);
    println!("is_even(42) => {is_even_result} | is_even(7) => {is_odd_result}");
    
    
    println!("\n=== 🧠 Testing Logic Functions ===");
    
    let max_3 = max_of_three(10, 25, 5);
    println!("max_of_three(10, 25, 5) => {max_3}");
    
    let numbers_array = [10, 8, 4, 8, 9, 6, 3, 4, 7, 8, 5, 2, 1, 3, 6];
    let array_max = max_of(&numbers_array);
    println!("max_of([array with 15 items]) => {array_max}");
    
    
    println!("\n=== 📝 Testing String Functions ===");
    
    let text = "   Rust is awesome   ";
    let word = first_word(text);
    println!("first_word(\"{text}\") => \"{word}\"");
    
    let vowels_text = "PRAGMATIC";
    let vowels_count = count_vowels(vowels_text);
    println!("count_vowels(\"{vowels_text}\") => {vowels_count}");

    let text_to_reverse = "Car";
    let reversed_text = reverse_string(text_to_reverse);
    println!("reverse_string(\"{text_to_reverse}\") => {reversed_text}");

    let custom_text = "Algorithm";
    let reversed_custom_text = custom_reverse_string(custom_text);
    println!("custom_reverse_string(\"{custom_text}\") => {reversed_custom_text}");

    println!("\n=== 🔢 Testing Advanced Mathematical Functions ===");

    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("celsius_to_fahrenheit({celsius}) => {fahrenheit}°F");

    let factorial_num = 5;
    println!("factorial({factorial_num}) => {}", factorial(factorial_num));

    let fibonacci_num = 10;
    println!("fibonacci({fibonacci_num}) => {}", fibonacci(fibonacci_num));
}
