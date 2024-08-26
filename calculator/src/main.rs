use std::io::BufRead;
use std::io;

struct Operation<'a> {
    op1: i32,
    op2: i32,
    operator: &'a str,
}

impl <'a>Operation<'a> {
    fn new(op1: i32, op2: i32, operator: &'a str) -> Operation<'a> {
        Self {
            op1: op1,
            op2: op2,
            operator: operator,
        }
    }

    fn calculate(&self) {
        match self.operator {
            "+" => {
                println!("{}", self.op1 + self.op2);
            },
            "-" => {
                println!("{}", self.op1 - self.op2);
            },
            "*" => {
                println!("{}", self.op1 * self.op2);
            },
            "/" => {
                match self.op2 {
                    0 => {
                        println!("Cannot divide by zero!");
                    },
                    _ => {
                        println!("{}", self.op1 as f64 / self.op2 as f64);
                    },
                }
            },
            "%" => {
                println!("{}", self.op1 % self.op2);
            },
            _ => {
                println!("Invalid operator!");
            }
        }
    }
}

fn main() {
    println!("Calculator in the format of : \"A operator B\"");
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let _ = handle.read_line(&mut buffer);

        let mut arr = buffer.split_whitespace();
        let o1= arr.next();
        let operator = arr.next();
        let o2= arr.next();
        match o1 {
            Some(test) => {
                if test == "exit" {
                    break;
                }
            }
            _ => {

            }
        }
        match (o1, o2, operator) {
            (Some(_), Some(_), Some(_)) => {
                //do nothing
            },
            _ => {
                println!("Not enough arguments!");
                continue;
            }
        }

        let o1 = o1.unwrap().parse();
        let o2 = o2.unwrap().parse();

        match (o1,o2) {
            (Ok(a), Ok(b)) => {
                let op = Operation::new(a, b, operator.unwrap());
                op.calculate();
            },
            _ => {
                println!("Invalid A or B");
                continue;
            }
        }


    };
}
