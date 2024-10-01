use core::str;
use std::{
    fs::{self, File, ReadDir},
    io::Write,
    path::{Path, PathBuf},
    slice::Iter,
};

use base64::{prelude::BASE64_STANDARD, Engine};

use crate::{check_list::CheckList, task::Task};

fn get_saves_path() -> PathBuf {
    let exe_path: PathBuf = std::env::current_exe().expect("Failed to get current executable path");
    let exe_directory: &Path = exe_path.parent().expect("Failed to get parent directory");
    let saves_path: PathBuf = exe_directory.join("saves");

    // Check if the "saves/" directory exists
    if !saves_path.exists() {
        // Create the directory if it does not exist
        if let Err(err) = fs::create_dir(&saves_path) {
            eprintln!("Failed to create 'saves/' directory: {}", err);
            return saves_path;
        }
    }
    return saves_path;
}

pub fn get_list_names() -> Vec<String> {
    let mut list_paths: Vec<String> = Vec::new();
    let saves_path: PathBuf = get_saves_path();

    // Try reading the directory and handle the error
    let paths: ReadDir = match fs::read_dir(saves_path) {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("Error reading saves directory: {}", err);
            return list_paths; // Return an empty list if there's an error
        }
    };

    // Iterate over directory entries
    for path in paths {
        match path {
            Ok(entry) => {
                let mut file_name = entry
                    .file_name()
                    .into_string()
                    .unwrap_or_else(|_| String::new());
                file_name = file_name.replace(".todo", "");
                list_paths.push(file_name.replace("_", " "));
            }
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
            }
        }
    }

    return list_paths; // Return the populated list
}

pub fn load_list(name: &str) -> CheckList {
    let file_name: String = name.replace(" ", "_");
    let file_path: PathBuf = get_saves_path().join(file_name + ".todo");
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
    let file_name: String = list.get_title().replace(" ", "_");
    let file_path: PathBuf = get_saves_path().join(file_name + ".todo");
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
