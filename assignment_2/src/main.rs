// Question.1
/*
use std::io;
fn main() {
    println!("Please input each term with spaces");
    let condition = true;
    while condition {
        let mut input = String ::new();
        io::stdin().read_line(& mut input).expect("Failed to read line");
        let mut term = vec![];
        for i in input.split_whitespace() {
         term.push(i);
    }
     let num1 :f32 = term[0].parse().unwrap();
     let num2 :f32 = term[2].parse().unwrap();
     if num1 == 0.0 { 
         println!("Bye ");
         break;
         }
     
    if term[1] == "+" { 
        println!("Answer = {}",num1+num2);
        }
    else if term[1] == "-" { 
        println!("Answer = {}",num1-num2);
        }
    else  if term[1] == "*" { 
        println!("Answer = {}",num1*num2);
        }
    else  if term[1] == "/" { 
        println!("Answer = {}",num1/num2);
        }
    else if term[1] == "^" {
        println!("Answer = {}",num1.powf(num2 as f32));
        }
    else { 
        println!("> Invalid Operator");
        }
    }  
}
*/

// Question.2
/*
fn main() {
    let x = || {"Hello World"}; //make a closure which takes no argument and prints hello world
    println!("{}",x());
}
fn main() {
    let x = |x :u32| { x+1 }; //Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
    let y = 5;
    println!("The function returns: {}",x(y)); 
}
fn main() {
    let c = 1;
    let x = || { c + 1}; //Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
    x();
    println!("The new value of c is: {}",c); // should print 2
}
// Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
fn hello_func<T:Fn()>(x:T)
{
    x();
}
fn main() 
{
     let x = || println!("Hello World");
     hello_func(x);
}
// Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
fn hello_func<T:Fn(u32)->u32>(x:T)->u32
{
     x(5)
}
fn main() 
{
     let x = |y| y+1;

     println!("{}",hello_func(x));
}
*/

// Question.3
/*
struct Data<T>
    where T: Fn(u32) -> u32
{
    Name : String,
    Age: T,
    Gender : String
}
impl<T> Data<T>
    where T: Fn(u32) -> u32
{
    fn new(Name:String,Age: T,Gender:String) -> Data<T> {
        Data {
            Name,
            Age,
            Gender
        }
    }
    fn print(&self) {
        println!("Name is {}",self.Name);
        println!("Age is {}",(self.Age)(25));
        println!("Gender is {}",self.Gender);
    }
}
fn main() {
    let name = String::from("Ali");
    let gender = String::from("Male");

    let data1 = Data::new(name,|num|num,gender);
    data1.print();
}
*/
// Question.4
/*
#[derive(Debug)]
struct Children {
   Name : String,
}

pub trait Primary_passed {
    fn primary(&self) ;
}
pub trait Bilingual {
    fn fn_bilingual(&self) ;
}

impl Primary_passed for Children {
    fn primary(&self) {}
}
impl Bilingual for Children {
    fn fn_bilingual(&self) {}
}
fn adopt<T>(child1:T,child2:T) 
    where T : Primary_passed + Bilingual
    {
        println!("Mr. Asim should adopt these two children");
    }
fn main () {
    let children1 = Children {
        Name :String::from("Ali"),
    };
    children1.primary();
    children1.fn_bilingual();
    let children2 = Children {
        Name :String::from("Babar"),
    };
    children2.primary();
    children2.fn_bilingual();

    adopt(children1,children2);
}
*/

// Question.5
/*
 1. Closures are anonymous functions.
 2. Closures can be saved in a variable or pass as arguments to other functions. 
 3. Closures can be created in one place and then called to evaluate it 
    in a different context. 
 4. Unlike functions, Closures can capture values from the scope in 
    which they’re defined. 
*/