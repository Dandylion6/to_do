use std::{
    fs::{self, File},
    io::Write,
};

use crate::{check_list::CheckList, task::Task};

pub fn get_list_paths() -> Vec<String> {
    let mut list_paths: Vec<String> = Vec::new();
    let paths = fs::read_dir("saves/").unwrap();
    for path in paths {
        list_paths.push(path.unwrap().file_name().into_string().unwrap());
    }
    return list_paths;
}

pub fn load_list(name: &str) -> CheckList {
    let file_path = "saves/".to_string() + name + ".txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut tasks = Vec::new();
    let mut buffer = String::new();

    for character in contents.bytes() {
        if character == b';' {
            tasks.push(Task::new(buffer.clone(), false));
            buffer.clear();
        } else {
            let char_string = character as char;
            buffer += char_string.to_string().as_str();
        }
    }

    return CheckList::load(name, tasks);
}

pub fn save_list(list: &CheckList) {
    let file_path = "saves/".to_string() + list.get_title() + ".txt";
    let mut file = File::create(file_path).unwrap();

    for task in list.get_tasks() {
        let task_line = task.get_description().to_string() + ";\n";
        file.write_all(task_line.as_bytes()).unwrap();
    }
}
