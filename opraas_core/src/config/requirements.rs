#[derive(Debug)]
enum Comparison {
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
pub struct Requirement {
    program: String,
    version: String,
    comparison: Comparison,
    get_version: fn() -> String,
}

pub fn get_requirements() -> Vec<Requirement> {
    vec![
        Requirement {
            program: "go".to_string(),
            version: "0.8.17".to_string(),
            comparison: Comparison::GreaterThanOrEqual,
            get_version: || {
                let output = std::process::Command::new("go")
                    .arg("version")
                    .output()
                    .expect("Failed to execute go command");
                let version = String::from_utf8_lossy(&output.stdout);
                version
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .get(2)
                    .unwrap()
                    .to_string()
            },
        },
    ]
}
