use std::{path::{PathBuf, Path}, time::Duration};

use super::util;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub serve_path: PathBuf,
    pub timeout: Duration,
    pub index_file_name: String,
    pub file_not_found_path: Box<Path>,
    pub is_in_debug_mode: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut index = 1;
        let mut port = 8080;
        let mut serve_path = PathBuf::from("./");
        let mut timeout = Duration::from_secs(30);
        let mut index_file_name = String::from("index.html");
        let mut file_not_found_path = serve_path.clone();
        file_not_found_path.push("404.html");
        let mut is_in_debug_mode = false;

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
                "--index-file-name" => {
                    if util::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing index file name"));
                    }

                    index_file_name = args[index + 1].to_string();
                    index += 1;
                },
                "--404-file" => {
                    if util::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing 404 file path"));
                    }

                    file_not_found_path = PathBuf::from(&args[index + 1]);
                    index += 1;
                },
                "--debug" => {
                    is_in_debug_mode = true;
                }
                _ => return Err(format!("Invalid parameter: \"{}\"", args[index])),
            }
            index += 1;
        }

        let serve_path = match serve_path.canonicalize() {
            Ok(serve_path) => serve_path,
            Err(_) => return Err(String::from("Failed to read directory")),
        };
        
        Ok(Config { 
            port,
            serve_path,
            timeout,
            index_file_name,
            file_not_found_path: Box::from(file_not_found_path),
            is_in_debug_mode,
        })
    }

}

#[cfg(test)]
mod tests {
    use std::{time::Duration, path::PathBuf};

    #[test]
    fn correctly_parses_arguments() {
        const PORT: u16 = 99;
        const TIMEOUT: u64 = 2;
        const SOURCE_FOLDER: &str = "./example";

        let args = [
            String::from("throwscape"),
            String::from("--port"), PORT.to_string(),
            String::from("--timeout"), TIMEOUT.to_string(),
            String::from("--source"), SOURCE_FOLDER.to_string(),
        ];
        
        let result = super::Config::new(&args).unwrap();

        assert_eq!(result.port, PORT);
        assert_eq!(result.timeout, Duration::from_secs(TIMEOUT));
        assert_eq!(result.serve_path, PathBuf::from(SOURCE_FOLDER).canonicalize().unwrap());
    }
}
