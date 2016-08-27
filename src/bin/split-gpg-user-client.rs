use std::process;

extern crate split_gpg_user;

use split_gpg_user::spawn_similarly;

const SERVER_BIN_NAME: &'static str = "split-gpg-user-server";

fn main() {
    let server = spawn_similarly(SERVER_BIN_NAME);
    let status = server.expect("Error running the server").wait().expect("Server errored");
    process::exit(status.code().unwrap_or(1));
}
