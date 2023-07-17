use std::env;
use std::process::Command;

fn exec(cmd: &str) -> String {
    let cmds: Vec<&str> = cmd.split(" ").collect();

    let op = Command::new("sh").args(&cmds).output().unwrap();

    if op.stdout.len() > 0 {
        return String::from_utf8_lossy(op.stdout.as_slice()).to_string();
    }

    let mut err = "Error: ".to_owned();
    err.push_str(
        String::from_utf8_lossy(op.stderr.as_slice())
            .to_string()
            .as_str(),
    );
    err
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        println!("Output:\n\n{}\n\n", exec(args[1].as_str()));
    } else {
        println!("[+] Usage:\n\n{} command\n\n", args[0]);
    }
}
