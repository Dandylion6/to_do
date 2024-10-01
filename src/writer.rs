use std::io::Write;

use crate::task::Task;

pub fn program_start() {
    println!("== Welcome to TO DO ==\n");
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

pub fn show_check_lists(lists: &Vec<String>) {
    println!("Please enter a number to open a check list.");
    println!("Check lists:\n");

    let mut i: usize = 1;
    lists.iter().for_each(|list: &String| {
        println!("{}. '{}'", i.to_string(), list);
        i += 1;
    });

    println!();
    add_element_option("check list");
}

pub fn add_element_option(element: &str) {
    println!("To add a new {element}, enter 'add'.");
}

pub fn return_option(return_to: &str) {
    println!("To go to {return_to}, enter 'back'.");
}

pub fn exit_program_option() {
    println!("To exit the program, enter 'exit'.");
}

pub fn show_list_tasks(tasks: &Vec<Task>) {
    let mut i: usize = 1;
    tasks.iter().for_each(|task: &Task| {
        println!("{}. '{}'", i.to_string(), task.get_description());
        i += 1;
    });

    println!();
    add_element_option("task");
}
