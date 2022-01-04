pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut index = 1;
        let mut port = 8080;

        while index < args.len() {
            match args[index].as_str() {
                "--port" => {
                    if args.len() <= index + 1 || args[index + 1].starts_with("--") {
                        return Err(String::from("Missing port number"));
                    }

                    let port_string = args[index + 1].to_string();
                    index += 1;

                    match port_string.parse::<u16>() {
                        Ok(num) => port = num,
                        Err(_) => return Err(String::from("Invalid port number")),
                    }
                },
                _ => return Err(format!("Invalid parameter: \"{}\"", args[index])),
            }
            index += 1;
        }
        
        Ok(Config { port })
    }
}
