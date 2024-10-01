use std::io::{stdout, Write};

use crate::task::Task;

pub fn program_start() {
    let title: &str = r#"
 _       __     __                             __           __________           ____  ____  __
| |     / /__  / /________  ____ ___  ___     / /_____     /_  __/ __ \         / __ \/ __ \/ /
| | /| / / _ \/ / ___/ __ \/ __ `__ \/ _ \   / __/ __ \     / / / / / /  ______/ / / / / / / / 
| |/ |/ /  __/ / /__/ /_/ / / / / / /  __/  / /_/ /_/ /    / / / /_/ /  /_____/ /_/ / /_/ /_/  
|__/|__/\___/_/\___/\____/_/ /_/ /_/\___/   \__/\____/    /_/  \____/        /_____/\____(_)   
                                                                                               "#;
    println!("{title}");
    println!("\nPlease press 'enter' to continue...");
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn ask_name_of(element: &str) {
    clear();
    println!("Please give new {element} a name.\n");
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

pub fn empty_container(element: &str) {
    println!("Uh oh! looks like you have no {element}, maybe try adding some?");
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

    println!("");
    add_element_option("task");
}
