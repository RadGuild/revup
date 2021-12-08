extern crate clap;
use clap::{App, Arg, ArgGroup};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{prelude::*};
use std::path::{Path, PathBuf};


#[derive(Serialize, Deserialize)]
struct Command {
    cmd: String,
    args: Vec<String>,
    envs: Vec<String>,
}

impl Command {
    fn new(command: &str, args: Vec<&str>, envs: Vec<&str>) -> Command {
        let args_owned: Vec<String> = ret_string_vec(args);
        let envs_owned: Vec<String> = ret_string_vec(envs);
        Command {
            cmd: command.to_string(),
            args: args_owned,
            envs: envs_owned,
        }
    }

    fn new_only_command(command: &str) -> Command {
        Command {
            cmd: command.to_string(),
            args: [].to_vec(),
            envs: [].to_vec(),
        }
    }

    fn new_no_args(command: &str, envs: Vec<&str>) -> Command {
        let envs_owned: Vec<String> = ret_string_vec(envs);
        Command {
            cmd: command.to_string(),
            args: [].to_vec(),
            envs: envs_owned,
        }
    }

    fn print(&self) {
        print!("\nCall:\n{}", self.cmd);
        for arg in &self.args {
            print!(" {}", arg);
        }
        if !self.envs.is_empty() {
            print!("\n With envs:\n");
            for env in &self.envs {
                print!("{} ", env);
            }
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Commands {
    commands: Vec<Command>,
}

fn main() {
    let matches =
        App::new("revup")
            .version("v0.2.0")
            .author("author: RadGuild")
            .about(
                "Sets up the resim simulator for calling functions instantly, looks for revup.json file in the current dir, and runs the resim commands in order storing the created entities address locations in a dotenv file. Run \">>> source .env\" after running revup and all your environment variables will be active in your shell.",
            )
            .arg(Arg::with_name("json_file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Use a custom json file"),
            ).arg(Arg::with_name("rev_file")
                .short("r")
                .long("rev")
                .takes_value(true)
                .help("Use a custom rev file or - for stdin"),
            ).arg(Arg::with_name("init")
                .short("i")
                .long("init")
                .help("Creates a default revup.json config file in the working directory"),
            ).arg(Arg::with_name("keep")
                .short("k")
                .long("keep")
                .help("Keeps the environment variables in the .env, useful when working with multiple revup.json files"),
            ).arg(Arg::with_name("list")
                .long("ls")
                .help("Lists all calls and envs"),
            ).arg(Arg::with_name("epoch")
                .short("e")
                .long("epoch")
                .takes_value(true)
                .min_values(0)
                .value_name("increment")
                .help("Increases the epoch by <increment>. If no <increment>, displays the current epoch"),
            ).group(ArgGroup::with_name("group")
                .args(&["json_file", "init", "list"])
                .required(false),
            )
            .get_matches();

    let mut keep = false;
    if matches.is_present("keep") {
        keep = true;
    }

    if matches.is_present("json_file") {
        let path = Path::new(matches.value_of("json_file").unwrap());
        match run_file(path.to_path_buf(), keep).err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    } else if matches.is_present("rev_file") {
        let filename = matches.value_of("rev_file").unwrap();
        // println!("rev file:|{}|", filename);
        let rdr: Box<dyn std::io::Read> = match filename {
            "-" => Box::new(std::io::stdin()),
            _ => Box::new(File::open(filename).expect("no such file")),
        };
        match run_rev_file(rdr).err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    } else if matches.is_present("init") {
        match run_init().err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    } else if matches.is_present("list") {
        match run_ls().err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    } else if matches.is_present("epoch") {
        let epoch_increment = matches.value_of("epoch");
        match run_epoch(epoch_increment).err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    } else {
        match run(keep).err() {
            Some(e) => println!("Critical error, aborting \n{}", e),
            None => {}
        }
    }
}

fn run(keep: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cur_dir = std::env::current_dir()?;
    cur_dir.push("revup.json");

    if !cur_dir.exists() {
        println!("revup.json file not found, run --init to create a default revup.json file");
        std::process::exit(0);
    }

    match run_file(cur_dir, keep).err() {
        Some(e) => println!("Error while executing commands \n{}", e),
        None => {}
    }
    Ok(())
}

fn run_file(path: PathBuf, keep: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !keep {
        let _dot_env = std::fs::File::create(".env")?;
        let _env_bat = std::fs::File::create("env.bat")?;
        let _env_ps1 = std::fs::File::create("env.ps1")?;
    }

    let file = std::fs::File::open(path)?;
    let json: Commands = serde_json::from_reader(file)?;

    for cmd in json.commands {
        run_cmd(cmd.cmd, cmd.args, cmd.envs)?;
    }

    Ok(())
}

fn run_rev_file(mut reader: Box<dyn std::io::Read>) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_text = String::new();
    match reader.read_to_string(&mut all_text).err() {
        Some(e) => println!("Error reading input, aborting \n{}", e),
        None => {}
    };
    let lines = all_text.split('\n');
    for line in lines {
        // println!("{:?}", line);
        let rawstr: Vec<&str> = line.splitn(2, "//").collect();
        // println!("{}", vstr[0]);
        let l = rawstr[0].trim();
        if l.len() > 0 {
            // println!("{}", l);
            if l.starts_with("#!") {
                continue;
            }
            let ca_e_str: Vec<&str> = l.splitn(2, " -> ").collect();
            // First extract the command string and the command args, if any 
            let c_a_str: Vec<&str> = ca_e_str[0].splitn(2, ' ').collect();
            let cmd: String = c_a_str[0].to_string();
            let mut args: Vec<String> = Vec::new();
            let mut envars: Vec<String> = Vec::new();

            if c_a_str.len() > 1 {
                let arg_string = c_a_str[1].trim().to_string();
                if arg_string.len() > 0 {
                    args = args_from_string(arg_string);
                }
            }
            
            if ca_e_str.len() > 1 {
                // We also have envvars to extract
                let ev: Vec<&str> = ca_e_str[1].split(' ').collect();
                for es in &ev[..] {
                    if es.len() > 0 {
                        envars.push(es.to_string());
                    }
                }
            }
            if cmd != "-e" {
                run_cmd(cmd, args, envars)?;
            } else {
                if c_a_str.len() > 1 {
                    run_epoch(Some(c_a_str[1]))?;
                } else {
                    run_epoch(None)?;
                }
            }
        }
    }

    Ok(())
}

fn run_init() -> Result<(), Box<dyn std::error::Error>> {
    let mut dir = std::env::current_dir()?;
    dir.push("revup.json");
    if !dir.exists() {
        match create_default_config_file() {
            Ok(_v) => println!("Default config file created in working directory"),
            Err(e) => println!("Error while creating config file \n{}", e),
        }
    } else {
        println!("revup.json file already exists remove it first, skipping");
    }

    Ok(())
}

fn run_ls() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = std::fs::File::open("revup.json")?;
    let json: Commands = serde_json::from_reader(json_file)?;

    println!("Command:");
    for command in json.commands {
        command.print();
    }
    println!("");
    let dot_env = std::fs::read_to_string(".env")?;
    println!("---------------------------------------------------------------------");
    println!(".env: \n{}", dot_env);
    Ok(())
}

fn run_epoch(epoch_increment: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let res = std::process::Command::new("resim").arg("show-configs").output()?;
    let stdout = String::from_utf8_lossy(&res.stdout);
    let current_epoch_line = stdout.lines()
        .nth(1)
        .unwrap();
    let current_epoch = current_epoch_line.trim()
        .split(" ")
        .last()
        .unwrap();

    match epoch_increment {
        Some(value) => {
            // increment the epoch
            let current = current_epoch.to_string().parse::<i32>()?;
            let increment = value.parse::<i32>()?;
            let new_epoch = current + increment;

            println!("Attempting to update epoch from {} to {}", current, new_epoch);
            let res = std::process::Command::new("resim")
                .args(["set-current-epoch", &new_epoch.to_string()])
                .output()?;
            println!("{}", String::from_utf8_lossy(&res.stdout).to_string().trim());
            println!("{}", String::from_utf8_lossy(&res.stderr).to_string());

            Ok(())
        },
        None => {
            // print the epoch
            println!("Current epoch: {}", current_epoch);
            Ok(())
        }
    }
}

fn run_cmd(
    command: String,
    args: Vec<String>,
    envs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // First deal with envvar substitutions
    // Replace $ with values from .env //Quick 'n Dirty no idea how this is going to behave on
    // non utf-8 systems , can somebody refactor this and create a proper method
    let mut args_vec: Vec<String> = Vec::new();
    for arg in args {
        if arg.contains("$") {
            let mut loc = arg.find("$").unwrap();
            loc += 1;
            let substr_arg = &arg[loc..];
            let find_string = substr_arg.to_string();

            //load env vars
            let dot_env = std::fs::read_to_string(".env")?;
            let env_lines = dot_env.lines();

            for line in env_lines {
                let env_var: Vec<&str> = line.split("=").collect();
                if env_var[0] == find_string {
                    loc -= 1;
                    let mut final_arg: String = arg[..loc].to_string();
                    final_arg.push_str(env_var[1]);
                    args_vec.push(final_arg);
                }
            }
        } else {
            args_vec.push(arg.to_string());
        }
    }

    let res;
    if !args_vec.is_empty() {
        print!(">>> {}", command);
        for arg in &args_vec {
            print!(" {} ", arg);
        }
        print!("\n");
        res = std::process::Command::new("resim")
            .arg(&command)
            .args(&args_vec)
            .output()?;
    } else {
        println!(">>> {}", command);
        res = std::process::Command::new("resim").arg(&command).output()?;
    }
    println!("{}", String::from_utf8_lossy(&res.stdout).to_string());
    println!("{}", String::from_utf8_lossy(&res.stderr).to_string());

    if !envs.is_empty() {
        let entities = walk_entities(String::from_utf8_lossy(&res.stdout).to_string())?;

        for (ent_it, env_it) in entities.iter().zip(envs.iter()) {
            println!("{}={}", env_it, ent_it);
            let _res = append_env(env_it.to_string(), ent_it.to_string())?;
        }
    }

    if command == "reset".to_string() {
        // reset the .env file
        let mut dot_env = std::fs::File::create(".env")?;
        let env: String = "tokenXRD=030000000000000000000000000000000000000000000000000004\n".to_string();
        Ok(dot_env.write_all(env.as_bytes())?)
    } else {
        Ok(())   
    }
}

fn walk_entities(stdout: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    //Quick and dirty
    let mut ret_vec: Vec<String> = Vec::new();
    let location: usize;

    match stdout.rfind("New Entities") {
        Some(loc) => location = loc,
        None => return Err("No entities found".into()),
    }

    let substr_entities = &stdout[location..];
    let lines: Vec<&str> = substr_entities.lines().collect();

    for line in lines {
        if line.contains(" Component: ")
            || line.contains(" ResourceDef: ")
            || line.contains(" Package: ")
            || line.contains("Public key:") // special case for new-account
        {
            let entity_vec: Vec<&str> = line.split_whitespace().collect();
            let entity = entity_vec[2].to_string();
            ret_vec.push(entity);
        }
    }

    if ret_vec.len() < 1 {
        return Err("No entities found".into());
    }

    Ok(ret_vec)
}

fn append_env(env: String, ent: String) -> Result<(), Box<dyn std::error::Error>> {
    // for bash
    let mut env_file = std::fs::OpenOptions::new().append(true).open(".env")?;
    let mut env_line = env.clone();
    env_line.push_str("=");
    env_line.push_str(&ent);
    env_line.push_str("\n");
    env_file.write_all(env_line.as_bytes())?;

    // for cmd
    let mut env_file = std::fs::OpenOptions::new().append(true).open("env.bat")?;
    let mut env_line = String::from("set ");
    env_line.push_str(&env);
    env_line.push_str("=\"");
    env_line.push_str(&ent);
    env_line.push_str("\"\r\n");
    env_file.write_all(env_line.as_bytes())?;

    // for ps
    let mut env_file = std::fs::OpenOptions::new().append(true).open("env.ps1")?;
    let mut env_line = String::from("$");
    env_line.push_str(&env);
    env_line.push_str("=\"");
    env_line.push_str(&ent);
    env_line.push_str("\"\r\n");
    env_file.write_all(env_line.as_bytes())?;

    Ok(())
}

fn create_default_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut commands_vec: Vec<Command> = Vec::new();

    commands_vec.push(Command::new_only_command("reset"));
    commands_vec.push(Command::new_no_args(
        "new-account",
        ["account", "pubkey"].to_vec(),
    ));
    commands_vec.push(Command::new_no_args(
        "new-account",
        ["account2", "pubkey2"].to_vec(),
    ));
    commands_vec.push(Command::new(
        "new-token-fixed",
        ["10000", "--name", "emunie", "--symbol", "EMT"].to_vec(),
        ["tokenEMT"].to_vec(),
    ));

    commands_vec.push(Command::new(
        "new-token-fixed",
        ["10000", "--name", "gmunie", "--symbol", "GMT"].to_vec(),
        ["tokenGMT"].to_vec(),
    ));

    commands_vec.push(Command::new(
        "publish",
        ["."].to_vec(),
        ["package"].to_vec(),
    ));

    println!("Enter the arguments for the first function call \nexample: BlueprintName new arg1 arg2 \n(No ticks, quotes or backticks)");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    let mut args_vec: Vec<&str> = s.split_whitespace().collect();
    args_vec.insert(0, "$package");

    println!("Enter the names of the env variables in correct order");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    let envs_vec: Vec<&str> = s.split_whitespace().collect();

    commands_vec.push(Command::new("call-function", args_vec, envs_vec));

    let commandos = Commands {
        commands: commands_vec,
    };

    let revup = std::fs::File::create("revup.json")?;
    let ret = serde_json::to_writer_pretty(revup, &commandos)?;
    Ok(ret)
}

fn ret_string_vec(vec: Vec<&str>) -> Vec<String> {
    let mut owned_vec: Vec<String> = Vec::new();
    for v in vec {
        owned_vec.push(v.to_string());
    }
    owned_vec
}

/*
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
*/

fn args_from_string(this_string: String) -> Vec<String> {
    let mut result: Vec<&str> = Vec::new();
    let top_split: Vec<&str> = this_string.split('"').collect();
    let mut quoted = false;
    for section in top_split {
        if quoted == false {
            let words: Vec<&str> = section.split_whitespace().collect();
            for w in words {
                let word = w.trim();
                if word.len() > 0 {
                    result.push(word);
                }
            }
            quoted = true;
        } else {
            result.push(section);
            quoted = false;
        }
    }
    ret_string_vec(result)
}
