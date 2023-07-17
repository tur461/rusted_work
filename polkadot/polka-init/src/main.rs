
use std::env;
use dotenv::dotenv;
use std::process::Command;
use clap::{Arg, ArgMatches, App, SubCommand};

const para_tmp_dir: &str = "para";
const relay_tmp_dir: &str = "relay";

fn main() {
    dotenv().ok();
    
    let tmp_dir = env::var("TMP_DIR").expect("TMP_DIR must be set");
    let specs_dir = env::var("SPECS_DIR").expect("SPECS_DIR must be set");
    let para_exec = env::var("PARA_BIN_PATH").expect("PARA_BIN_PATH must be set");
    let relay_exec = env::var("RELAY_BIN_PATH").expect("RELAY_BIN_PATH must be set");
    
    println!("specs_dir {:?}", specs_dir);

    let matches = get_arg_matches();
    
    if matches[0] == 1 {
        generate_para_spec_files(specs_dir, para_exec);
    } else if matches[1] == 1 {
        spawn_para_nodes(specs_dir, para_exec);
    } else if matches[2] == 1 {
        spawn_relay_nodes(specs_dir, para_exec);
    } else if matches[3] == 1 {
        post_spawn(specs_dir, para_exec);
    } else if matches[4] == 1 {
        clean(tmp_dir);
    } else {
        println!("no option provided!!");
    }

}

fn generate_para_spec_files(specs_dir: String, para_exec: String) {
    println!("generating spec files for para-chain/s..");
}

fn spawn_para_nodes(specs_dir: String, para_exec: String) {
    println!("spawning para node/s..");
}

fn spawn_relay_nodes(specs_dir: String, para_exec: String) {
    println!("spawning relay node/s..");
}

fn post_spawn(specs_dir: String, para_exec: String) {
    println!("doing post-spawn procedures..");
}

fn clean(tmp_dir: String) {
    println!("cleaning shit..");
    let output = Command::new("rm")
        .args([
            "-r", 
            "-f", 
            format!("../{}/{}", tmp_dir, para_tmp_dir).as_str(),
            format!("../{}/{}", tmp_dir, relay_tmp_dir).as_str(),
        ])
        .output()
        .expect("Failed to execute command");
        println!("done!!");
    // println!("{}", String::from_utf8_lossy(&output.stdout));
    // assert_eq!(b"Hello world\n", output.stdout.as_slice());
}


fn get_arg_matches() -> Vec<u64> {
    let matches = App::new("polka-init")
        .version("0.1")
        .author("tur461 <tur461@github>")
        .about("Bootstrap relay-para nodes!")
        .arg(Arg::with_name("para_spec")
            .short('c')
            .help("generate plain and raw spec files for para-chain"))
        .arg(Arg::with_name("para_run")
            .short('p')
            .help("run para node/s"))
        .arg(Arg::with_name("relay_run")
            .short('r')
            .help("run relay node/s"))
        .arg(Arg::with_name("post_spawn")
            .short('a')
            .help("do post spawn procedure"))
        .arg(Arg::with_name("clean")
            .short('x')
            .help("clean environment for new start"))
        .get_matches();

    vec![
        matches.occurrences_of("para_spec"),
        matches.occurrences_of("para_run"),
        matches.occurrences_of("relay_run"),
        matches.occurrences_of("post_spawn"),
        matches.occurrences_of("clean")
    ]
}