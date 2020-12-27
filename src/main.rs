// fn : function => specific task
mod flow;

fn main() {
    // _print_format()
    // _data_type();
    // _shadowing();
    // _test_param(10, 21);
    // flow::print();
    _loops_flow();
}
fn _loops_flow() {
    // loop {
    //     println!("again..!");
    // }
    let mut number = 3;

    // while number != 0 {
    //     println!("{}", number);
    //     number = number - 1;
    // }

    // Compound : array or tuple
    let numbers = [10, 20, 30, 40, 50];
    println!("{}", numbers[0]);
    
    for number in numbers.iter(){
        println!("{}", number);

    }
    // let tup = (20, "heloo");
    // println!("{}", tup.0);

}

fn _test_param(marks: u8, age: u8) {
    println!("marks {} : age {}", marks, age);
}

fn _shadowing() {
    // let mut marks = 20;
    // marks = 30;

    let marks = 20;
    let marks = 30;

    println!("{}", marks);
}
fn _data_type() {
    // Data Types
    // ints: u/i8, u/i16, u/i32, u/i64, u/isize
    let marks: u32 = 20; // int
    let mark2 = marks;
    println!("{}", mark2);

    // floats: f32, f64
    let marks: f32 = 20.1; // int
    println!("{}", marks);

    // bool
    let status = true; // int
    println!("{}", status);
}
fn _print_format() {
    //println!("Hello, world!");
    // _another_function();
    // println!("{} : {} : {}", "Ali", "wali", "khan");
    // println!("{0} : {2} : {1}", "Ali", "wali", "khan");
    // println!(
    //     "{name3} has a pen. {name2} picks flowers. {name1} is good boy",
    //     name1 = "Ali",
    //     name2 = "wali",
    //     name3 = "khan"
    // );

    // println!("{:b}", 20);
    // println!("{:x}", 20);
    // println!("{:o}", 20);
}
// custom
fn _another_function() {
    println!("Working....!");
}
