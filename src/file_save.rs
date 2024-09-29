use core::str;
use std::{
    fs::{self, File, ReadDir},
    io::{self, Write},
    slice::Iter,
};

use base64::{prelude::BASE64_STANDARD, Engine};

use crate::{check_list::CheckList, task::Task};

pub fn get_list_paths() -> Vec<String> {
    let mut list_paths: Vec<String> = Vec::new();
    let paths: ReadDir = fs::read_dir("saves/").unwrap();

    paths.for_each(|path: Result<fs::DirEntry, io::Error>| {
        list_paths.push(path.unwrap().file_name().into_string().unwrap());
    });
    return list_paths;
}

pub fn load_list(name: &str) -> CheckList {
    let file_path: String = "saves/".to_string() + name + ".txt";
    let decoded: Vec<u8> = BASE64_STANDARD
        .decode(fs::read(file_path).unwrap())
        .unwrap();

    let mut tasks: Vec<Task> = Vec::new();
    let mut buffer: String = String::new();

    let mut is_value_field: bool = false;
    let mut is_task_done: bool = false;
    for character in decoded {
        let is_task_end: bool = character == b';';
        let is_desc_end: bool = character == b':';

        if is_desc_end || is_task_end {
            if is_task_end {
                tasks.push(Task::new(buffer.clone(), is_task_done));
                buffer.clear();

                is_value_field = false;
                is_task_done = false;
            } else if !is_value_field {
                is_value_field = true;
            }
            continue;
        }

        if is_value_field && character.is_ascii_digit() {
            is_task_done = (character - b'0') > 0;
        } else {
            let char_string: char = character as char;
            buffer += char_string.to_string().as_str();
        }
    }
    return CheckList::load(name, tasks);
}

pub fn save_list(list: &CheckList) {
    let file_path: String = "saves/".to_string() + list.get_title() + ".txt";
    let mut file: File = File::create(file_path).unwrap();

    let iterator: Iter<'_, Task> = list.get_tasks().into_iter();
    iterator.for_each(|task: &Task| {
        let done_value: &str = if task.is_done() { "1" } else { "0" };
        let mut encoded: String = String::new();
        BASE64_STANDARD.encode_string(
            task.get_description().to_string() + ":" + done_value + ";",
            &mut encoded,
        );
        file.write_all(encoded.as_bytes()).unwrap();
    });
}
