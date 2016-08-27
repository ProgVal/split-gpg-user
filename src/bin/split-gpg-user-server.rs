use std::env;
use std::process;
extern crate split_gpg_user;

use split_gpg_user::*;

const GPG_BIN_NAME: &'static str = "/usr/bin/gpg";
const ZENITY_BIN_NAME: &'static str = "/usr/bin/zenity";

fn main() {
    let question = format!("Allow call of GPG with the following args? {:?}", env::args().skip(1).collect::<Vec<String>>());
    let user_consents = ask_user_consent(ZENITY_BIN_NAME, "An application request access to GPG", &question);
    let user_consents = user_consents.expect("Zenity errored");
    if user_consents {
        let gpg = spawn_similarly(GPG_BIN_NAME);
        let status = gpg.expect("Error running GPG").wait().expect("GPG errored");
        process::exit(status.code().unwrap_or(1));
    }
    else {
        println!("Access to GPG denied by user.");
        process::exit(1);
    }
}
