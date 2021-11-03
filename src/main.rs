extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let matches =
        App::new("revup v0.0.1")
            .version("0.0.1")
            .author("dRAT3")
            .about(
                "

Sets up the rev2 simulator for calling functions instantly, looks for .revup file
in the current dir, if it can't be found uses the config file in the standard
config directory of your OS. 

Currently windows isn't supported. Pull requests for windows are welcome!

",
            )
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .takes_value(true)
                    .help("Uses a custom .revup file"),
            )
            .arg(
                Arg::with_name("init")
                    .short("i")
                    .help("Creates a default config file in the working directory"),
            )
            .arg(Arg::with_name("reset").short("r").help(
                "Resets the simulator, creates a new account and stores the value in $account",
            ))
            .group(
                ArgGroup::with_name("group")
                    .args(&["file", "reset", "init"])
                    .required(false),
            )
            .get_matches();

    if matches.is_present("file") {
        let path = Path::new(matches.value_of("file").unwrap());
        run_file(path.to_path_buf());
    } else if matches.is_present("reset") {
        run_reset();
    } else if matches.is_present("init") {
        run_init();
    } else {
        match run().err() {
            Some(e) => println!("{}", e),
            None => {}
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cur_dir = std::env::current_dir()?;
    cur_dir.push(".revup");

    let file;

    if cur_dir.exists() {
        file = cur_dir.to_path_buf();
        println!("Using {:?}", file);
    } else {
        let config_file = return_config_path()?;
        if config_file.exists() {
            file = config_file;
            println!("Using {:?}", file);
        } else {
            println!(".revup file not found in working directory or config directory \n\rCreating default config file in {:?}",
               config_file);
            let _res = create_default_config_file(config_file)?;
        }
    }
    Ok(())
}
fn run_file(path: PathBuf) {}

fn run_reset() {
    //Reset ledger state
    println!(">>>rev2 reset");
    let reset = Command::new("rev2")
        .arg("reset")
        .output()
        .expect("failed to execute rev2");
    println!("{}", String::from_utf8_lossy(&reset.stdout).to_string());
    println!("{}", String::from_utf8_lossy(&reset.stderr).to_string());

    assert!(reset.status.success());

    println!(">>>rev2 new-account");
    //Create account and export value
    let create = Command::new("rev2")
        .arg("new-account")
        .output()
        .expect("failed to execute rev2");
    println!("{}", String::from_utf8_lossy(&create.stdout).to_string());
    println!("{}", String::from_utf8_lossy(&create.stderr).to_string());
    assert!(create.status.success());

    //Might not work on windows
    let res = walk_create(String::from_utf8_lossy(&create.stdout).to_string());

    let mut account;
    match res {
        Ok(v) => {
            account = v;
        }
        Err(e) => {
            println!("Couldn't find account, exiting");
            std::process::exit(1);
        }
    }
    /*
    let mut arg = "account=".to_string();
    arg.push_str(&account);
    let export = Command::new("export")
        .arg(arg)
        .output()
        .expect("Unable to set var");
    */
}

fn run_init() {
    match std::env::current_dir() {
        Ok(mut dir) => {
            dir.push(".revup");
            match create_default_config_file(dir) {
                Ok(_v) => println!("Default config file created in working directory"),
                Err(e) => println!("Error while creating config file \n\r{}", e),
            }
        }
        Err(e) => println!("Error: couldn't find working directory \n\r{}", e),
    }
}

fn walk_create(stdout: String) -> Result<String, Box<dyn std::error::Error>> {
    //Quick and dirty todo:change unwrap() and return err
    let loc_entities = stdout.rfind("New Entities").unwrap();

    let substr_entities = &stdout[loc_entities..];

    println!("{}", substr_entities);
    Ok(stdout)
}

fn read_file(path: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cmd_vec = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(cmd) = line {
                cmd_vec.push(cmd);
            }
        }
    }
    if cmd_vec.len() == 0 {
        return Err("Error: file is empty".into());
    }
    Ok(cmd_vec)
}

fn create_default_config_file(file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn return_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("revup");
            Ok(path)
        }
        None => Err("Config dir not found".into()),
    }
}
