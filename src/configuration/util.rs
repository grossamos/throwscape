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

#[cfg(test)]
mod tests {
    #[test]
    fn missing_value_check_catches_missing_value() {
        let args = [String::from("--port"), String::from("--wrong-val")];
        let index = 0;
        assert_eq!(true, super::check_for_missing_next_value(&args, index));
    }

    #[test]
    fn missing_value_check_doesnt_catch_correct_values() {
        let args = [String::from("--port"), String::from("8080")];
        let index = 0;
        assert_eq!(false, super::check_for_missing_next_value(&args, index));
    }
}

