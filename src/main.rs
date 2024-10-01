pub mod check_list;
pub mod file_save;
pub mod program;
pub mod task;
pub mod writer;

fn main() {
    writer::program_start();
    program::startup();
}
