use std::{path::PathBuf, time::Duration};

use super::util;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub serve_path: PathBuf,
    pub timeout: Duration
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut index = 1;
        let mut port = 8080;
        let mut serve_path = PathBuf::from("./");
        let mut timeout = Duration::from_secs(30);

        while index < args.len() {
            match args[index].as_str() {
                "--port" => {
                    if util::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing port number"));
                    }

                    port = util::parse_next_arg(args, index)?;
                    index += 1;

                },
                "--source" => {
                    if util::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing source path"));
                    }

                    serve_path = PathBuf::from(&args[index + 1]);
                    index += 1;
                },
                "--timeout" => {
                    if util::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing timeout in sec"));
                    }
                    
                    timeout = Duration::from_secs(util::parse_next_arg(args, index)?);
                    index += 1;
                },
                _ => return Err(format!("Invalid parameter: \"{}\"", args[index])),
            }
            index += 1;
        }
        
        Ok(Config { 
            port,
            serve_path,
            timeout,
        })
    }

}

