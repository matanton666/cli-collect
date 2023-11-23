use cli_collect::utils::{self, print_err_str, DEF_COLOR, read_config_file, print_warning, print_err};
use cli_collect::color_str::ColoredStr;
use cli_collect::table::Table;

use std::fs::Permissions;
use std::path::PathBuf;
use std::{path::Path};
use std::io::{Result, Error};
use chrono::{DateTime, Utc};
use humansize::{format_size, DECIMAL};
use serde::{Serialize, Deserialize};
use clap::Parser;

static TOOL_NAME: &'static str = "ils";
// defaults
static DEFAULT_DATE_FORMAT: &'static str = "%d/%m/%Y %H:%M";
static DEFAULT_DIR_COLOR: &'static str = DEF_COLOR;
static DEFAULT_FILE_COLOR: &'static str = DEF_COLOR;



#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// directory to list
    #[arg(default_value_t=String::from("."))]
    dir: String,

    /// show more info
    #[arg(short, long)]
    verbose: bool,

    // TODO: sort
    // TODO: -d load default values without config file
}

/// entry type (file, directory, simbolic link...)
enum EntryType {
    DIR,
    FILE,
    SYMLINK,
}

/// an entry in the list. this is the parameters it has
struct Entry {
    permitions: String,
    creation_date: DateTime<Utc>,
    modification_date: DateTime<Utc>,
    size: u64,
    name: String,
    entry_type: EntryType,
    // TODO: make enum for type : dir, file, hidden file, simbol link...
}

#[derive(Serialize, Debug, Deserialize)]
/// the configurations for displaying the list of entrys
struct DisplayConf {
    date_format: String,
    dir_color: String,
    file_color: String,
    // TODO: symlink, size and date colors
    show: DisplayColumns,
}

#[derive(Serialize, Debug, Deserialize)]
/// which columns to show and which to hide when displaying the list
struct DisplayColumns {
    permitions: bool,
    creation_date: bool,
    modification_date: bool,
    size: bool,
}


fn main() {
    let args = Args::parse();
    let mut ils: Vec<Entry> = Vec::new();

    // create default configuration to put in file incase it does not exist 
    let default_config = DisplayConf {
        date_format: DEFAULT_DATE_FORMAT.to_string(),
        dir_color: DEFAULT_DIR_COLOR.to_string(),
        file_color: DEFAULT_FILE_COLOR.to_string(),
        show: DisplayColumns { 
            permitions: true, creation_date: true, modification_date: true, size: true 
        }
    };

    // get configurations form file or create one if not there
    let config: DisplayConf = match utils::read_config_file(TOOL_NAME, &default_config) {
        Ok(conf) => conf,
        Err(err) => {
            utils::print_err(err);
            utils::print_warning("using default configuration");
            default_config
        }
    };

    let mut path = PathBuf::from(&args.dir);

    // list file
    loop {
        if path.is_file() {
            print_err_str("u trying to list a file???");
            std::process::exit(1);
        }

        let ils: Vec<Entry> = list_dir(&path).unwrap_or_default();
        let table = get_entrys_as_table(&config, &ils);

        // print talbe and get choice of user back
        let choice = table.print_interactive();
        if choice < 0 { // chose to exit
            table.print_as_table();
            break;
        }

        // get the name of the next choice entry
        let row = table.get(choice as u32);
        if let Some(name) = row.get("name") {
            path.push(name.clone());
        } 
        else {
            print_err_str("can find name");
            break;
        }
    }


}

fn list_dir(dir: &Path) -> Result<Vec<Entry>> {
    let mut ils: Vec<Entry> = Vec::new();

    // add the parent directory
    if let Some(entry) = dir.parent(){
        let metadata =entry.metadata()?;

        let entry = Entry {
            permitions: permitions_to_string(metadata.permissions()),
            creation_date: DateTime::from(metadata.created()?),
            modification_date: DateTime::from(metadata.modified()?),
            size: 0,
            entry_type: EntryType::DIR,
            name: String::from(".."),
        };
        ils.push(entry);
    }

    for entity in dir.read_dir()? {
        let entity = entity?;
        let metadata = entity.metadata()?;

        let entry = Entry {
            permitions: permitions_to_string(metadata.permissions()),
            creation_date: DateTime::from(metadata.created()?),
            modification_date: DateTime::from(metadata.modified()?),
            size: metadata.len(),
            entry_type: get_entry_type(metadata),
            name: String::from(entity.file_name().to_str().unwrap()),
        };
        ils.push(entry);
    }


    return Ok(ils);
}


fn get_entrys_as_table(config: &DisplayConf, entry_list: &Vec<Entry>) -> Table {
    let mut table = Table::new();

    for entry in entry_list {

        // * init all column values
        let permitions = ColoredStr::new(
            entry.permitions.clone());
        let creation_date = ColoredStr::new(
            entry.creation_date.format(&config.date_format).to_string());
        let modification_date = ColoredStr::new(
            entry.modification_date.format(&config.date_format).to_string());
        let mut size = ColoredStr::new(
            humansize::format_size(entry.size, DECIMAL));
        let mut name = ColoredStr::new_colored(
            entry.name.clone(), config.file_color.clone());

        // * the modifications to the coulmns:
        match entry.entry_type {
            EntryType::DIR => {
                name.append("/");
                name.set_color(config.dir_color.clone());
                size.clear();
            }
            EntryType::SYMLINK => {
                name.append(" -> link");
                // TODO: symlink color
            }
            EntryType::FILE => {},
        }
        
        // * what to show
        if config.show.permitions {
            table.insert("permittions", &permitions);
        }
        if config.show.creation_date {
            table.insert("created", &creation_date);
        }
        if config.show.modification_date {
            table.insert("modified", &modification_date);
        }
        if config.show.size {
            table.insert("size", &size);
        }
        table.insert("name", &name)

    }

    return table;
}


fn permitions_to_string(permissions: Permissions) -> String {
    if permissions.readonly() {
        return String::from("r-");
    }
    else {
        return String::from("rw");
    }
}

fn get_entry_type(metadata: std::fs::Metadata) -> EntryType {
    if metadata.is_dir() {
        return EntryType::DIR;
    }
    else if metadata.is_symlink() {
        return EntryType::SYMLINK;
    } 

    EntryType::FILE
}