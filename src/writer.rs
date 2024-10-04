use std::{
    io::{stdout, Write},
    str::FromStr,
};

use crate::{
    program::get_status_select_input,
    task::{Status, Task},
};

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
        println!("{}. {}", i.to_string(), list);
        i += 1;
    });

    if lists.is_empty() {
        empty_container("check lists");
    }
    println!();
    add_element_option("check list");
}

pub fn add_element_option(element: &str) {
    println!("To add a new {element}, enter 'add'.");
}

pub fn delete_element_option(element: &str) {
    println!("To delete this {element}, enter 'delete'");
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
    println!("Please enter a number to modify a task.");
    println!("Tasks:\n");

    let mut i: usize = 1;
    tasks.iter().for_each(|task: &Task| {
        let status: String = get_status_string(task.get_status());
        println!("{}. {} - {}", i.to_string(), task.get_description(), status);
        i += 1;
    });

    if tasks.is_empty() {
        empty_container("tasks");
    }
    println!("");
    add_element_option("task");
}

pub fn show_task_modify_options(task_name: &str) {
    println!("Please enter a valid status for task '{}'.", task_name);
    println!("Options:\n");
    Status::into_iter(Status::Planned).for_each(|status: Status| {
        println!(
            "'{}' -> {}",
            get_status_select_input(status),
            get_status_string(status)
        );
    });
    println!("");
}

fn get_status_string(status: Status) -> String {
    match status {
        Status::Planned => return String::from_str("PLANNED").unwrap(),
        Status::Doing => return String::from_str("DOING").unwrap(),
        Status::Done => return String::from_str("DONE").unwrap(),
        Status::Cancelled => return String::from_str("CANCELLED").unwrap(),
    }
}
