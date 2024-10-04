use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use crate::{
    check_list::CheckList,
    file_save,
    task::{Status, Task},
    writer::{self, clear},
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
            load_list(&input);
        }
    }
}

fn try_open_list(index: usize, file_names: &Vec<String>) {
    let mut failed: bool = false;
    if index == 0 || index > file_names.len() {
        failed = true;
    }

    if failed {
        return check_list_menu();
    }
    let file_name: &String = file_names.get(index - 1).unwrap();
    load_list(file_name.as_str());
}

fn load_list(file_name: &str) {
    let mut list: CheckList = file_save::load_list(file_name);
    open_list(&mut list);
}

fn open_list(list: &mut CheckList) {
    let tasks: &Vec<Task> = list.get_tasks();
    clear();

    writer::show_list_tasks(&tasks);
    writer::delete_element_option("check list");
    writer::return_option("check lists");

    let input: String = get_input();
    match input.as_str() {
        "add" => add_new_task(list),
        "delete" => delete_list(list),
        "back" => check_list_menu(),
        _ if input.chars().all(|character: char| character.is_numeric()) => {
            let index: usize = input.parse::<usize>().unwrap();
            try_modify_task(index, list);
        }
        _ => open_list(list),
    }
}

fn delete_list(list: &CheckList) {
    file_save::delete_list(list);
    check_list_menu();
}

fn add_new_task(list: &mut CheckList) {
    clear();
    writer::ask_name_of("task");
    writer::return_option("check list");

    let input: String = get_input();
    match input.as_str() {
        "back" => open_list(list),
        _ if input.is_empty() => add_new_task(list),
        _ => {
            let new_task: Task = Task::new(input.as_str(), list.get_title());
            list.add_task(new_task);
            file_save::save_list(list);
            open_list(list);
        }
    }
}

fn try_modify_task(index: usize, list: &mut CheckList) {
    let tasks: &mut Vec<Task> = list.get_tasks_mut();
    let mut failed: bool = false;
    if index == 0 || index > tasks.len() {
        failed = true;
    }

    if failed {
        return open_list(list);
    }
    let task: &mut Task = tasks.get_mut(index - 1).unwrap();
    modify_task(task);
    if task.is_deleted() {
        list.remove_task(index - 1);
    }
    file_save::save_list(list);
    return open_list(list);
}

fn modify_task(task: &mut Task) {
    clear();
    writer::show_task_modify_options(task.get_description());
    writer::delete_element_option("task");
    writer::return_option("check list");

    let mut input: String = get_input();
    match input.as_str() {
        "back" => load_list(task.get_list_name()),
        "delete" => delete_task(task),
        _ if input.is_empty() => return,
        _ => {
            Status::into_iter(Status::Planned).for_each(|status: Status| {
                input = input.to_lowercase();
                let to_match: String = get_status_select_input(status);
                let matches: bool = input.as_bytes() == to_match.as_bytes();
                if matches {
                    return task.set_status(status);
                }
            });
        }
    }
}

fn delete_task(task: &mut Task) {
    task.mark_for_deletion();
}

pub fn get_status_select_input(status: Status) -> String {
    match status {
        Status::Planned => return String::from_str("pln").unwrap(),
        Status::Doing => return String::from_str("dig").unwrap(),
        Status::Done => return String::from_str("dne").unwrap(),
        Status::Cancelled => return String::from_str("cld").unwrap(),
    }
}
