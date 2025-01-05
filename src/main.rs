fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    let var = "world!";
    println!("Hello, {}", var);
}

#[test]
fn mutable_test() {
    let mut var = "world!";
    println!("Hello, {}", var);

    var = "test";
    println!("Hello, {}", var);
}

#[test]
fn shadowing_test() {
    let mut var = "world!";
    println!("Hello, {}", var);

    var = "test";
    println!("Hello, {}", var);
}

#[test]
fn explicit() {
    let age : i32 = 30;
    let name : &str = "John";
    println!("{} is {}", name, age);
}

#[test]
fn explicit_1() {
    let number : i8 = 10;
    let float : f32 = 10.4;
    println!("{} is {}", number, float);
}

#[test]
fn numeric_opt() {
    let a = 10;
    let b = 10;
    let c = a + b;
    println!("result a + b = {}", c);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 10;
    let c = a == b;
    println!("result a == b = {}", c);
}

#[test]
fn comparison_1() {
    let a = 10;
    let b = 10;
    let c = a > b;
    println!("result a > b = {}", c);
}

#[test]
fn char_test() {
    let char1: char = 'a';
    let char2: char = 'b';
    let char3: char = 'c';
    println!("{} {} {}", char1, char2, char3);
}

#[test]
fn tuple_test() {
    let mut tup: (i32, f64, bool) = (500, 6.4, true);
    println!("{:?}", tup);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("{} {} {}", a, b, c);

    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    //mutable
    tup = (501, 6.4, true);
    let (a, _, _) = tup;
    println!("{}", a);
}

#[test]
fn array_test() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let a = arr[0];
    println!("{}", a);
}

#[test]
fn two_dimensional_array_test() {
    let arr: [[i32; 3]; 2] = [
        [3, 4, 5], 
        [3, 4, 5]
        ];
    println!("{:?}", arr);
    println!("{}", arr[0][0]);
}

#[test]
fn variable_scope() {
    let a = 10;

    {
        //inner scope
        let b = a;
        println!("{}", b);
    }
    println!("{}", a);
}

#[test]
fn string() {
    let name: &str = "  John  ";
    let trim: &str = name.trim();
    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name: String = String::from("John");
    name.push_str(" Doe");
    println!("{}", name);

    let budi = name.replace("John", "Budi");
    println!("{}", budi);
}

#[test]
fn clone() {
    let name: String = String::from("John");
    let clone = name.clone();
    println!("{}", name);
    println!("{}", clone);
}

#[test]
fn if_expression() {
    let a = 10;
    let b = 20;
    if a < b {
        println!("a is less than b");
    } else if a > b {
        println!("a is greater than b");
    } else {
        println!("a is equal to b");
    }
}

#[test]
fn loop_expression() {
    let mut a = 0;
    while a < 10 {
        println!("{}", a);
        a += 1;
    }
}

#[test]
fn loop_expression_1() {
    let mut a = 0;
    loop {
        println!("counter : {}", a);
        a += 1;
        if a == 10 {
            break;
        }
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("start : {}", range.start);
    println!("end : {}", range.end);

    let array: [&str; 5] = ["a", "b", "c", "d", "e"];

    for i in 0..5 {
        println!("{}", array[i]);
    }
}

fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let full_name = full_name(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut name = String::from("John");
    change_value(&mut name);
    println!("{}", name);
}

#[test]
fn match_value() {
    let name: &str = "John";
    match name {
        "John" | "Budi" => {
            println!("betul");
        }
        other => {
            println!("tidak betul : {}", other);
        }
    }
}

#[test]
fn match_expression() {
    let name: &str = "John";
    let result = match name {
        "John" | "Budi" => "betul",
        _ => "tidak betul"
    };
    println!("{}", result);
}

#[test]
fn test_iterator_method(){
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5,6,7,8,9,10];
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("{}", sum);

    let count: usize = vector.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 == 1).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        Some(host) => {
            println!("Connected to database at{}", host);
        }
        None => {
            panic!("No host provided");
        }
    }
}

#[test]
fn test_unrecoverable_error() {
    let host = Some(String::from("localhost"));
    connect_database(host);
    // connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => {
            Ok(host)
        }
        None => {
            Err(String::from("No cache host provided"))
        }
    }
}

#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(Some(String::from("localhost")));
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Connected to cache at {}", host);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}