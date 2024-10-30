use colored::*;

pub fn info(msg: impl AsRef<str>) {
    println!("{}", msg.as_ref().white().bold());
}

pub fn warn(msg: impl AsRef<str>) {
    println!("{}", msg.as_ref().yellow().bold());
}

pub fn error(msg: impl AsRef<str>) {
    println!("{}", msg.as_ref().red().bold());
}

pub fn success(msg: impl AsRef<str>) {
    println!("{}", msg.as_ref().green().bold());
}
