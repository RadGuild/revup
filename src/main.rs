extern crate clap;
use clap::{App, Arg, ArgGroup};
use dotenv;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Commando {
    command: String,
    args: Vec<String>,
    envs: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Commandos {
    commands: Vec<Commando>,
}

fn main() {
    let matches =
        App::new("revup")
            .version("v0.0.1")
            .author("author: dRAT3")
            .about(
                "
Sets up the rev2 simulator for calling functions instantly, looks for revup.json file
in the current dir, and runs the rev2 commands in order storing the created entities
address locations in a dotenv file. Run \">>> source .env\" after running revup and all 
your environment variables will be active in your shell.

Currently windows isn't supported. Pull requests for windows are welcome!
",
            )
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .takes_value(true)
                    .help("Uses a custom revup.json file"),
            )
            .arg(Arg::with_name("init").short("i").help(
                "Creates a default config file in the working directory, and the envup.sh file",
            ))
            .arg(Arg::with_name("keep")
                .short("k")
                .help("Keeps the environment variables in the .env, useful when working with multiple revup.json files"))
            .group(
                ArgGroup::with_name("group")
                    .args(&["file", "init"])
                    .required(false),
            )
            .get_matches();

    let mut keep = false;
    if matches.is_present("keep") {
        keep = true;
    }

    if matches.is_present("file") {
        let path = Path::new(matches.value_of("file").unwrap());
        match run_file(path.to_path_buf(), keep).err() {
            Some(e) => println!("{}", e),
            None => {}
        }
    } else if matches.is_present("init") {
        run_init();
    } else {
        match run(keep).err() {
            Some(e) => println!("{}", e),
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
    }

    let file = std::fs::File::open(path)?;
    let json: Commandos = serde_json::from_reader(file)?;

    for cmd in json.commands {
        dotenv::dotenv().ok();
        //Replace $ with values from .env //Quick 'n Dirty no idea how this is going to behave on
        //non utf-8 systems , one big mess, can somebody refactor this and create a proper
        //function?
        let mut args_vec: Vec<String> = Vec::new();
        for arg in &cmd.args {
            if arg.contains("$") {
                let mut loc = arg.find("$").unwrap();
                loc += 1;
                let substr_arg = &arg[loc..];
                let find_string = substr_arg.to_string();

                for (key, value) in std::env::vars() {
                    if key == find_string {
                        println!("Found var {}", find_string);
                        if loc > 0 {
                            loc -= 1;
                            let mut final_arg: String = arg[..loc].to_string();
                            final_arg.push_str(&value);
                            args_vec.push(final_arg);
                        } else {
                            args_vec.push(value);
                        }
                    }
                }
            } else {
                args_vec.push(arg.to_string());
            }
        }
        run_cmd(cmd.command, args_vec, cmd.envs)?;
    }

    Ok(())
}

fn run_init() {
    match std::env::current_dir() {
        Ok(mut dir) => {
            dir.push("revup.json");
            if !dir.exists() {
                match create_default_config_file() {
                    Ok(_v) => println!("Default config file created in working directory"),
                    Err(e) => println!("Error while creating config file \n{}", e),
                }
            } else {
                println!("revup.json file already exists remove it first, skipping");
            }
        }
        Err(e) => println!("Error: couldn't access working directory \n{}", e),
    }
}

fn run_cmd(
    command: String,
    args: Vec<String>,
    envs: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let res;
    if !args.is_empty() {
        print!(">>> {}", command);
        for arg in &args {
            print!(" {} ", arg);
        }
        print!("\n");
        res = Command::new("rev2").arg(&command).args(&args).output()?;
    } else {
        println!(">>> {}", command);
        res = Command::new("rev2").arg(&command).output()?;
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
    Ok(())
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

fn append_env(mut env: String, ent: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut dotenv = std::fs::OpenOptions::new().append(true).open(".env")?;
    env.push_str("=");
    env.push_str(&ent);
    env.push_str("\n");
    Ok(dotenv.write_all(env.as_bytes())?)
}

fn create_default_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut vector: Vec<Commando> = Vec::new();
    let reset = Commando {
        command: "reset".to_owned(),
        args: [].to_vec(),
        envs: [].to_vec(),
    };
    vector.push(reset);

    let account = Commando {
        command: "new-account".to_owned(),
        args: [].to_vec(),
        envs: ["account".to_string()].to_vec(),
    };
    vector.push(account);

    let token1 = Commando {
        command: "new-resource-fixed".to_owned(),
        args: [
            "10000".to_string(),
            "--name".to_string(),
            "emunie".to_string(),
            "--symbol".to_string(),
            "EMT".to_string(),
        ]
        .to_vec(),
        envs: ["token1".to_string()].to_vec(),
    };
    vector.push(token1);

    let token2 = Commando {
        command: "new-resource-fixed".to_owned(),
        args: [
            "10000".to_string(),
            "--name".to_string(),
            "gmunie".to_string(),
            "--symbol".to_string(),
            "GMT".to_string(),
        ]
        .to_vec(),
        envs: ["token2".to_string()].to_vec(),
    };
    vector.push(token2);

    let publish = Commando {
        command: "publish".to_owned(),
        args: [".".to_string()].to_vec(),
        envs: ["package".to_string()].to_vec(),
    };
    vector.push(publish);

    println!("Enter the arguments for the first function call \nexample: PackageName new 200,$token1 200,$token2 \nDon't use \" or \' !");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    let mut a: Vec<&str> = s.split_whitespace().collect();
    a.insert(0, "$package");

    let mut arguments_owned: Vec<String> = Vec::new();
    for i in a {
        arguments_owned.push(i.to_string());
    }

    println!("Enter the names of the env variables in correct order");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    let e: Vec<&str> = s.split_whitespace().collect();

    let mut envs_owned: Vec<String> = Vec::new();

    for i in e {
        envs_owned.push(i.to_string());
    }

    let first_function = Commando {
        command: "call-function".to_owned(),
        args: arguments_owned,
        envs: envs_owned,
    };

    vector.push(first_function);

    let commandos = Commandos { commands: vector };

    let revup = std::fs::File::create("revup.json")?;
    let ret = serde_json::to_writer_pretty(revup, &commandos)?;
    Ok(ret)
}
