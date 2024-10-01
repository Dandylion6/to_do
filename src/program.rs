use std::io::{stdin, stdout, Write};

use crate::{
    check_list::CheckList, file_save, task::Task, writer::{self, clear}
};

pub fn get_input() -> String {
    print!("\n> ");
    stdout().flush().unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().to_string().to_lowercase();
}

pub fn startup() {
    get_input();
    check_list_menu();
}

pub fn check_list_menu() {
    let list_names: Vec<String> = file_save::get_list_names();

    clear();
    writer::show_check_lists(&list_names);
    if list_names.is_empty() {
        writer::empty_container("check lists");
    }
    writer::exit_program_option();

    let input: String = get_input();
    match input.as_str() {
        "add" => create_new_list(),
        "exit" => return,
        _ if input.chars().all(|character: char| character.is_numeric()) => {
            let index: usize = input.parse::<usize>().unwrap();
            try_open_list(index, &list_names);
        }
        _ => check_list_menu(),
    }
}

fn create_new_list() {
    clear();
    writer::ask_name_of("check list");
    writer::return_option("check lists");
    writer::exit_program_option();

    let input: String = get_input();
    match input.as_str() {
        "exit" => return,
        "back" => check_list_menu(),
        _ if input.is_empty() => create_new_list(),
        _ => {
            let new_list = CheckList::new(&input);
            file_save::save_list(&new_list); // Save newly created list
            open_list(&input);
        }
    }
}

fn try_open_list(index: usize, file_names: &Vec<String>) {
    let mut failed: bool = false;
    if index == 0 || index > file_names.len() {
        failed = true;
    }

    if failed {
        check_list_menu();
    } else {
        let file_name: &String = file_names.get(index - 1).unwrap();
        open_list(file_name.as_str());
    }
}

fn open_list(file_name: &str) {
    let list: CheckList = file_save::load_list(file_name);
    let tasks: &Vec<Task> = list.get_tasks();
    clear();

    if tasks.is_empty() {
        writer::empty_container("tasks");
    }
    writer::show_list_tasks(&tasks);
    writer::return_option("check lists");

    let input: String = get_input();
    match input.as_str() {
        "add" => create_new_list(),
        "back" => check_list_menu(),
        _ => open_list(file_name),
    }
}
