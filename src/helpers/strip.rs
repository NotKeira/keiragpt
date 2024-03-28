pub struct Parameters<'a> {
    flags: Vec<&'a str>,
    args: Vec<&'a str>,
}

pub fn split_flags(args: Vec<&str>) -> Parameters<'_> {
    let mut flags = Vec::new();
    let mut rest = Vec::new();
    for arg in args {
        if arg.starts_with("-") {
            flags.push(&arg[1..]);
        } else {
            rest.push(arg);
        }
    }
    Parameters { flags, args: rest }
}