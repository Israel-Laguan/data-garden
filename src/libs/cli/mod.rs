pub mod config;
pub mod env;
pub mod logger;
pub mod types;

use crate::libs::cli::{env::apply_env, logger::init_logger};
use crate::libs::local_fs::{check_disk_space, file_exists};
use clap::{App, Arg, ArgMatches};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();

    if std::env::args().len() == 1 {
        println!(
            r"
        ______        ____   ,---------.    ____              .-_'''-.      ____    .-------.     ______         .-''-.  ,---.   .--. 
        |    _ `''.  .'  __ `.\          \ .'  __ `.          '_( )_   \   .'  __ `. |  _ _   \   |    _ `''.   .'_ _   \ |    \  |  | 
        | _ | ) _  \/   '  \  \`--.  ,---'/   '  \  \        |(_ o _)|  ' /   '  \  \| ( ' )  |   | _ | ) _  \ / ( ` )   '|  ,  \ |  | 
        |( ''_'  ) ||___|  /  |   |   \   |___|  /  |        . (_,_)/___| |___|  /  ||(_ o _) /   |( ''_'  ) |. (_ o _)  ||  |\_ \|  | 
        | . (_) `. |   _.-`   |   :_ _:      _.-`   |        |  |  .-----.   _.-`   || (_,_).' __ | . (_) `. ||  (_,_)___||  _( )_\  | 
        |(_    ._) '.'   _    |   (_I_)   .'   _    |        '  \  '-   .'.'   _    ||  |\ \  |  ||(_    ._) ''  \   .---.| (_ o _)  | 
        |  (_.\.' / |  _( )_  |  (_(=)_)  |  _( )_  |         \  `-'`   | |  _( )_  ||  | \ `'   /|  (_.\.' /  \  `-'    /|  (_,_)\  | 
        |       .'  \ (_ o _) /   (_I_)   \ (_ o _) /          \        / \ (_ o _) /|  |  \    / |       .'    \       / |  |    |  | 
        '-----'`     '.(_,_).'    '---'    '.(_,_).'            `'-...-'   '.(_,_).' ''-'   `'-'  '-----'`       `'-..-'  '--'    '--' 
                                                                                                                                       
        "
        );
        return Ok(());
    }

    if matches.is_present("version") {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let is_quiet = matches.is_present("quiet");
    init_logger(is_quiet);

    let project_name = matches.value_of("project-folder").unwrap_or_default();

    apply_env();

    if let Some(project_name) = matches.value_of("init") {
        init_project(project_name)?;
    } else if let Some(dataset) = matches.value_of("process") {
        process_dataset(dataset)?;
    } else if matches.is_present("server") {
        start_server()?;
    }

    Ok(())
}

fn parse_args() -> ArgMatches {
    App::new("Data Garden: grooming your datasets with ❤️")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Israel Laguan <israellaguan@gmail.com>")
        .about("Processes datasets using different templates")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .takes_value(false)
                .help("Start in server mode"),
        )
        .arg(
            Arg::new("project-folder")
                .short('f')
                .takes_value(true)
                .help("Name of the project in snake case"),
        )
        .arg(
            Arg::new("process")
                .short('p')
                .long("process")
                .takes_value(true)
                .help("Process a dataset"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .takes_value(false)
                .help("Run quietly with minimal output"),
        )
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .takes_value(true)
                .help("Initialize a new project"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .takes_value(false)
                .help("Show version information"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .takes_value(false)
                .help("Show help information"),
        )
        .get_matches()
}

fn init_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = format!("projects/{}", project_name);
    std::fs::create_dir_all(&project_dir)?;

    let config_content = read_config("./config.yaml")?;
    let config_data = serde_yml::to_string(&config_content)?;

    std::fs::write(format!("{}/config.yaml", project_dir), config_data)?;

    println!("Initialized project: {}", project_name);
    Ok(())
}

fn process_dataset(dataset_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_path = "/home/israelllaguan/";
    let available_space = check_disk_space(home_path);

    println!(
        "Available space in {}: {} bytes",
        home_path, available_space
    );
    let dataset_file_exists = file_exists(dataset_path);
    println!("Dataset located in: {dataset_path} {dataset_file_exists}");
    Ok(())
}

fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // Web server starting logic here
    Ok(())
}
