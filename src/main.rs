fn main() {
    let event: &str = "FossMeet24";
    println!("Hello, {}!", event);

    let n = 22;

    if n > 18 {
        print!("Adult");
    } else {
        print!("Not adult");
    }

    let x = 5;
    println!("x has the value {}", x);

    let y = 10;
    if y == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    println!("{}", greet());
    println!("{}", greetings("FOSSMeet24"));
}

fn greet() -> String {
    return "Hello Shravan Kumar B".to_string();
}

fn greetings(to: &str) -> String {
    let hi = "Hello Shravan Kumar B to";

    return format!("{} {}", hi, to);
}
