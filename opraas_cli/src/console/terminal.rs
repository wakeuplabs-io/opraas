use colored::Colorize;

pub fn print_info(msg: &str) {
    println!("{}", msg.bright_white().bold());
}

pub fn print_error(msg: &str) {
    eprintln!("{}", msg.bold().red());
}

pub fn print_success(msg: &str) {
    println!("{}", msg.bold().green());
}

pub fn print_warning(msg: &str) {
    println!("{}", msg.bold().yellow());
}
