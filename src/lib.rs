use std::env;
use std::process::{Command, Stdio, Child};
use std::os::unix::io::FromRawFd;
use std::io;

pub fn spawn_similarly(bin_name: &str) -> Result<Child, io::Error> {
    let mut command = Command::new(bin_name);

    // Apply the caller's stdin/stdout/stderr to the callee
    let command = command
                       .stdin(unsafe { Stdio::from_raw_fd(0) })
                       .stdout(unsafe { Stdio::from_raw_fd(1) })
                       .stderr(unsafe { Stdio::from_raw_fd(2) });

    // Apply all arguments of the caller to the callee
    let command = env::args().skip(1).fold(command, |acc, arg| acc.arg(arg));

    command.spawn()
}

pub fn ask_user_consent(zenity_bin_name: &str, title: &str, question: &str) -> Result<bool, io::Error> {
    let mut zenity = Command::new(zenity_bin_name);

    let zenity = zenity
                       .arg("--question")
                       .arg("--title")
                       .arg(title)
                       .arg("--text")
                       .arg(question)
                       
                       // Hide complaints about accessing the accessibility bus and GTK warnings
                       .stderr(Stdio::null());

    zenity.spawn()
          .and_then(|mut child| child.wait())
          .map(|status| status.success())
}
