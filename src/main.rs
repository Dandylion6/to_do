use check_list::CheckList;
use file_save::{load_list, save_list};
use task::Task;

pub mod check_list;
pub mod file_save;
pub mod task;

fn main() {
    let list = load_list("hello!");
    for task in list.get_tasks() {
        print!("{}", task.get_description());
    }
}
