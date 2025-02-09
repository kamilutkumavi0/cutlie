use std::process::Stdio;
use std::process::Command;

pub fn run(command: &str){
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    let _ = child.wait().expect("Command wasn't running");
    
}
