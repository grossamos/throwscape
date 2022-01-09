use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub serve_path: PathBuf,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut index = 1;
        let mut port = 8080;
        let mut serve_path = PathBuf::from("./");

        while index < args.len() {
            match args[index].as_str() {
                "--port" => {
                    if Self::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing port number"));
                    }

                    let port_string = args[index + 1].to_string();
                    index += 1;

                    match port_string.parse::<u16>() {
                        Ok(num) => port = num,
                        Err(_) => return Err(String::from("Invalid port number")),
                    }
                },
                "--source" => {
                    if Self::check_for_missing_next_value(args, index) {
                        return Err(String::from("Missing source path"));
                    }

                    serve_path = PathBuf::from(&args[index + 1]);
                    index += 1;
                }
                _ => return Err(format!("Invalid parameter: \"{}\"", args[index])),
            }
            index += 1;
        }
        
        Ok(Config { 
            port,
            serve_path,
        })
    }

    fn check_for_missing_next_value(args: &[String], index: usize) -> bool {
        args.len() <= index + 1 || args[index + 1].starts_with("--")
    }
}
