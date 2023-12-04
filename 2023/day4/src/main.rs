mod task_one;
mod task_two;

use std::fs;

use task_one::execute as execute_one;
use task_two::execute as execute_two;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("Task 1 Result: {0}", execute_one(contents.clone()));
    println!("Task 2 Result: {0}", execute_two(contents.into()));
}
