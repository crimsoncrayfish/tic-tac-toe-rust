use std::str::FromStr;

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

    out
}
