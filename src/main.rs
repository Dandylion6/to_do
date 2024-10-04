use std::env;

use winresource::WindowsResource;

pub mod check_list;
pub mod file_save;
pub mod program;
pub mod task;
pub mod writer;

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("icon.ico")
            .compile()
            .unwrap();
    }
    writer::program_start();
    program::startup();
}
