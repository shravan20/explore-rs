struct Car {
    price: i32,
    model: String,
    manufacturer: String,
}

fn main() {
    // hello world
    let event: &str = "FossMeet24";
    println!("Hello, {}!", event);

    let n = 22;

    // simple if
    if n > 18 {
        print!("Adult");
    } else {
        print!("Not adult");
    }

    let x = 5;
    println!("x has the value {}", x);

    // if else
    let y = 10;
    if y == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    println!("{}", greet());
    println!("{}", greetings("FOSSMeet24"));

    // Vectors
    let v: Vec<i32> = vec![1, 2, 3];
    println!("Vector V = {}", format!("{:?}", v));

    let mut cars: Vec<Car> = Vec::new();
    let c = Car {
        price: 100,
        model: "Civic".into(),
        manufacturer: "Honda".into(),
    };
    cars.push(c);
}

// function
fn greet() -> String {
    return "Hello Shravan Kumar B".to_string();
}

// parametric function
fn greetings(to: &str) -> String {
    let hi = "Hello Shravan Kumar B to";

    return format!("{} {}", hi, to);
}
