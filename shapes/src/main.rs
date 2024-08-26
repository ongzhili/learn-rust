use std::f64::consts::PI;
use std::io::BufRead;
use std::io;
use std::process;

trait Shape {
    fn get_area(&self) -> f64;
    fn no_of_sides(&self) -> i32;
}

struct Square {
    length: f64,
}

struct Rectangle {
    length: f64,
    height: f64,
}

struct Trapezium {
    length_a: f64,
    length_b: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn get_area(&self) -> f64 {
        self.length * self.length
    }

    fn no_of_sides(&self) -> i32 {
        4
    }
}

impl Square {
    fn new(length: f64) -> Self {
        Self {
            length:length,
        }
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.length * self.height
    }

    fn no_of_sides(&self) -> i32 {
        4
    }
}

impl Rectangle {
    fn new(length: f64, height: f64) -> Self {
        Self {
            length: length,
            height: height,
        }
    }
}

impl Shape for Trapezium {
    fn get_area(&self) -> f64 {
        (self.length_a * self.length_b) * self.height * 0.5
    }

    fn no_of_sides(&self) -> i32 {
        4
    }
}

impl Trapezium {
    fn new(length_a: f64, length_b: f64, height: f64) -> Self {
        Self {
            length_a: length_a,
            length_b: length_b,
            height: height,
        }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn no_of_sides(&self) -> i32 {
        1
    }
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self {
            radius: radius,
        }
    }

}

fn get_shape_info(shpe: &impl Shape) {
    println!("Area: {}, No. of Sides: {}\n", shpe.get_area(), shpe.no_of_sides());
}

fn print_selected_shape(s: &String) {
    match s.trim() {
        "" => {
            println!("You have not selected a shape, please choose one:");
        },
        "circle" => {
            println!("Please input the radius:");
        },
        "square" => {
            println!("Please input the side length:");
        },
        "trapezium" => {
            println!("Please input the top length, bottom length, and height:");
        },
        "rectangle" => {
            println!("Please input the length and height:");
        },
        "exit" => {
            process::exit(1);
        }
        _ => {
            println!("Invalid Shape");
        }
    }
}

fn main() {
    
    println!("Welcome to the shapes program! Currently, we have the following shapes:\n\n\tCircle, Square, Rectangle, Trapezium\n\n");
    let mut decided_shape: String = String::from("");

    loop {
        print_selected_shape(&decided_shape);

        let mut buffer: String = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let _ = handle.read_line(&mut buffer);
        match decided_shape.trim() {
            "square" => {
                let mut args = buffer.split_whitespace();
                let length = args.next().expect("aa").parse::<f64>().unwrap();
                let square1 = Square::new(length);
                get_shape_info(&square1);
                decided_shape = String::from("");
            },
            "rectangle" => {
                let mut args = buffer.split_whitespace();
                let length = args.next().expect("aa").parse::<f64>().unwrap();
                let height = args.next().expect("aa").parse::<f64>().unwrap();
                let rectangle1 = Rectangle::new(length, height);
                get_shape_info(&rectangle1);
                decided_shape = String::from("");
            },
            "circle" => {
                let mut args = buffer.split_whitespace();
                let radius = args.next().expect("aa").parse::<f64>().unwrap();
                let circle1 = Circle::new(radius);
                get_shape_info(&circle1);
                decided_shape = String::from("");
            },
            "trapezium" => {
                let mut args = buffer.split_whitespace();
                let length_a = args.next().expect("aa").parse::<f64>().unwrap();
                let length_b = args.next().expect("aa").parse::<f64>().unwrap();
                let height = args.next().expect("aa").parse::<f64>().unwrap();
                let trapezium1 = Trapezium::new(length_a, length_b, height);
                get_shape_info(&trapezium1);
                decided_shape = String::from("");
            },
            _ => decided_shape = buffer.trim().to_string(),
        }



    }
}
