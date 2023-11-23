use std::{io::{Error, Result}, process, fs};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use toml;
use colored::{Colorize, ColoredString};

pub static DEF_COLOR: &'static str = "white";


pub fn print_err(err: Error) {
    eprintln!("ERROR: {} - \n{}", err.kind().to_string(), err.to_string().red());
}

pub fn print_err_w_exit(err: Error) {
    print_err(err);
    process::exit(1);
}

pub fn print_err_str(err: &str) {
    eprintln!("ERROR: {}", err.red());
}

pub fn print_warning(warn: &str) {
    println!("WARNING: {}", warn.yellow());
}



pub fn read_config_file<T: Serialize  + for<'d> Deserialize<'d>>(tool_name: &str, default_conf: &T) -> Result<T> {
    if let Some(proj_dir) = ProjectDirs::from(
        "dev", 
        "",
        "Cli-Collect") 
    {
        // get conf file path
        let conf_path = proj_dir.config_dir().join(tool_name);

        // create directory and file if they don't exist
        if let Some(parent_dir) = conf_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        if !conf_path.exists() {
            let default_content = toml::to_string(&default_conf).unwrap_or("".to_string());
            fs::write(&conf_path, default_content)?;
        }

        // read from file
        let config_content = fs::read_to_string(&conf_path)?;
        let config: T = match toml::from_str(&config_content) {
            Ok(conf) => conf,
            Err(_) => return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not parse the contents of the config file"
            ))
        };
        
        Ok(config)
    }
    else {
        Err(Error::new(std::io::ErrorKind::Other, "could not find the config default directory"))
    }

}

pub fn color_string(input: &String, color: &str) -> ColoredString {
    match color {
        "red" => input.red(),
        "green" => input.green(),
        "blue" => input.blue(),
        "cyan" => input.cyan(),
        "magenta" => input.magenta(),
        "yellow" => input.yellow(),
        "black" => input.black(),
        "white" => input.white(),
        "purple" => input.purple(),
        "bright_red" => input.bright_red(),
        "bright_green" => input.bright_green(),
        "bright_blue" => input.bright_blue(),
        "bright_cyan" => input.bright_cyan(),
        "bright_magenta" => input.bright_magenta(),
        "bright_yellow" => input.bright_yellow(),
        "bright_black" => input.bright_black(),
        "bright_white" => input.bright_white(),
        _ => {
            eprintln!("{} is not a valid color", color);
            input.normal()
        }
    }
}


