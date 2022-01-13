use std::str::FromStr;

pub fn check_for_missing_next_value(args: &[String], index: usize) -> bool {
    args.len() <= index + 1 || args[index + 1].starts_with("--")
}

pub fn parse_next_arg<T>(args: &[String], index: usize) -> Result<T, String>
    where
        T: FromStr,
{
    let next_arg = args[index + 1].to_string();

    match next_arg.parse::<T>() {
        Ok(num) => Ok(num),
        Err(_) => return Err(String::from("Invalid port number")),
    }

}

