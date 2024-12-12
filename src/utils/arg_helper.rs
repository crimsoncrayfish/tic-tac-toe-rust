use std::str::FromStr;

///Read an argument from the input args list
///
/// # Arguments
///
/// * `args` - all args received from the input
/// * `arg` - name of the argument
/// * `default` - the default value in case the argument was not passed in
///
/// # Returns
///
/// Argument value with the correct type.
/// NB Types have to implement FromStr
///
/// # Examples
///
/// ```
/// let args: Vec<String> = env::args().collect();
/// let _x_len: usize = read_config(&args, "--x-len".to_string(), 10);
///
/// ```
pub fn read_config<T>(args: &Vec<String>, arg: String, default: T) -> T
where
    T: FromStr + Clone + Copy,
{
    let mut out: T = default.clone();
    for i in 1..args.len() {
        if args[i].as_str() != arg {
            continue;
        }
        if let Some(val) = args.get(i + 1) {
            out = val.parse().unwrap_or_else(|_| {
                eprint!("invalid value for {}. Using default.", arg);
                default
            });
        }
    }
    print!("reading args");

    out
}

#[cfg(test)]
mod tests {
    use super::read_config;

    #[test]
    fn read_success() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("--test".to_string());
        args.push("100".to_string());

        let x: usize = read_config(&args, "--test".to_string(), 10000);
        assert_eq!(x, 100, "Expected value of 100");
    }

    #[test]
    fn read_failed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("--test2".to_string());
        args.push("100".to_string());

        let x: usize = read_config(&args, "test".to_string(), 10000);
        assert_eq!(x, 10000, "Expected value of 100");
    }
}
