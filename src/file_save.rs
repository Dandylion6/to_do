use std::{
    fs::{self, File},
    io::Write,
};

use crate::{check_list::CheckList, task::Task};

pub fn load_list(name: &str) -> CheckList {
    let file_path = "saves/".to_string() + name + ".txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't read file!");

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
    let mut file = File::create(file_path).expect("Couldn't create file!");

    for task in list.get_tasks() {
        let task_line = task.get_description().to_string() + ";\n";
        file.write_all(task_line.as_bytes())
            .expect("Couldn't write to file!");
    }
}
