use semver::Version;

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
    version: Version,
    comparison: Comparison,
    get_version: fn() -> Version,
}

pub fn get_requirements() -> Vec<Requirement> {
    vec![Requirement {
        program: "docker".to_string(),
        version: Version::parse("24.0.0").unwrap(),
        comparison: Comparison::GreaterThanOrEqual,
        get_version: || {
            let output = std::process::Command::new("docker")
                .arg("-v")
                .output()
                .expect("Failed to get docker version");
            Version::parse(
                String::from_utf8_lossy(&output.stdout)
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .get(2)
                    .unwrap()
                    .to_string()
                    .strip_suffix(",")
                    .unwrap_or("-"),
            )
            .unwrap()
        },
    }]
}

pub fn check_requirements() -> Result<(), String> {
    let requirements = get_requirements();
    for requirement in requirements {
        let version = (requirement.get_version)();
        match requirement.comparison {
            Comparison::Equal => {
                if version != requirement.version {
                    return Err(format!(
                        "Version {} does not equal required version {}",
                        version, requirement.version
                    ));
                }
            }
            Comparison::GreaterThanOrEqual => {
                if version < requirement.version {
                    return Err(format!(
                        "Version {} is not greater than or equal to required version {}",
                        version, requirement.version
                    ));
                }
            }
            Comparison::LessThanOrEqual => {
                if version > requirement.version {
                    return Err(format!(
                        "Version {} is not less than or equal to required version {}",
                        version, requirement.version
                    ));
                }
            }
            Comparison::GreaterThan => {
                if version <= requirement.version {
                    return Err(format!(
                        "Version {} is not greater than required version {}",
                        version, requirement.version
                    ));
                }
            }
            Comparison::LessThan => {
                if version >= requirement.version {
                    return Err(format!(
                        "Version {} is not less than required version {}",
                        version, requirement.version
                    ));
                }
            }
        }
    }

    Ok(())
}
