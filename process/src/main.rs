use std::process::Command;

fn main() {
    // Prints output to stdout by default
    Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("failed to start new process");
    println!("This will be printed first since the above command is ran asynchronously.");

    // Print out a message from an environment variable. This is ran synchronously.
    let output = Command::new("sh")
        .env("MESSAGE", "hello")
        .args(["-c", "echo $MESSAGE"])
        .output()
        .expect("failed to start new process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));


    let command = "ls -la | grep Cargo";
    println!("Running arbitrary command");
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to start new process");
}
