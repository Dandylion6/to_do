use std::io::{stdin, stdout, Write};

use crate::{
    check_list::CheckList,
    file_save,
    writer::{self, clear_terminal},
};

pub fn start() {
    writer::program_start();
    check_list_menu();
}

fn get_input() -> String {
    print!("\nYour input: ");
    std::io::stdout().flush().unwrap();

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string().to_lowercase();
}

fn check_list_menu() {
    let list_names: Vec<String> = file_save::get_list_names();

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
        _ => {
            clear_terminal();
            check_list_menu();
        }
    }
}

fn create_new_list() {}

fn try_open_list(index: usize, file_names: &Vec<String>) {
    let mut failed: bool = false;
    if index == 0 || index > file_names.len() {
        failed = true;
    }

    if failed {
        clear_terminal();
        check_list_menu();
    } else {
        let file_name: &String = file_names.get(index - 1).unwrap();
        open_list(file_name.as_str());
    }
}

fn open_list(file_name: &str) {
    let list: CheckList = file_save::load_list(file_name);
    clear_terminal();

    writer::show_list_tasks(&list.get_tasks());
    writer::return_option("check lists");

    let input: String = get_input();
    match input.as_str() {
        "add" => create_new_list(),
        "back" => check_list_menu(),
        _ => open_list(file_name),
    }
}
