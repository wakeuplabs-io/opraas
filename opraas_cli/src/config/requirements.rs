use crate::utils::{System, TSystem};
use assert_cmd::Command;
use regex::Regex;
use semver::Version;

#[derive(Debug)]
pub enum Comparison {
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
pub struct Requirement<'a> {
    pub program: &'a str,
    pub version_arg: &'a str,
    pub required_version: Version,
    pub required_comparator: Comparison,
}

pub struct SystemRequirementsChecker {
    system: Box<dyn TSystem>,
}

pub trait TSystemRequirementsChecker {
    fn check(self, requirements: Vec<Requirement>) -> Result<(), String>;
}

impl SystemRequirementsChecker {
    pub fn new() -> Self {
        Self {
            system: Box::new(System::new()),
        }
    }
}

impl TSystemRequirementsChecker for SystemRequirementsChecker {
    fn check(self, requirements: Vec<Requirement>) -> Result<(), String> {
        for requirement in requirements.iter() {
            let output = self.system.execute_command(
                &mut Command::new(requirement.program).arg(requirement.version_arg),
            )?;
            let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();

            let version = Version::parse(
                &re.captures(&output)
                    .ok_or(format!("Failed to parse version from output: {}", output))?[1],
            )
            .unwrap();

            match requirement.required_comparator {
                Comparison::Equal => {
                    if version != requirement.required_version {
                        return Err(format!(
                            "Version {} does not equal required version {}",
                            version, requirement.required_version
                        ));
                    }
                }
                Comparison::GreaterThanOrEqual => {
                    if version < requirement.required_version {
                        return Err(format!(
                            "Version {} is not greater than or equal to required version {}",
                            version, requirement.required_version
                        ));
                    }
                }
                Comparison::LessThanOrEqual => {
                    if version > requirement.required_version {
                        return Err(format!(
                            "Version {} is not less than or equal to required version {}",
                            version, requirement.required_version
                        ));
                    }
                }
                Comparison::GreaterThan => {
                    if version <= requirement.required_version {
                        return Err(format!(
                            "Version {} is not greater than required version {}",
                            version, requirement.required_version
                        ));
                    }
                }
                Comparison::LessThan => {
                    if version >= requirement.required_version {
                        return Err(format!(
                            "Version {} is not less than required version {}",
                            version, requirement.required_version
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::requirements::Comparison;
    use crate::config::requirements::Requirement;
    use crate::config::requirements::SystemRequirementsChecker;
    use crate::utils::MockTSystem;

    #[test]
    fn test_check() {
        let requirements = vec![Requirement {
            program: "rustc",
            version_arg: "--version",
            required_version: Version::new(1, 0, 0),
            required_comparator: Comparison::GreaterThanOrEqual,
        }];

        let mut mock_system = MockTSystem::new();
        mock_system
            .expect_execute_command()
            .times(1)
            .returning(|_| Ok("rustc 1.0.0".to_string()));

        let checker = SystemRequirementsChecker {
            system: Box::new(mock_system),
        };
        let result = checker.check(requirements);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_fails() {
        let requirements = vec![Requirement {
            program: "rustc",
            version_arg: "--version",
            required_version: Version::new(1, 0, 0),
            required_comparator: Comparison::GreaterThanOrEqual,
        }];

        let mut mock_system = MockTSystem::new();
        mock_system
            .expect_execute_command()
            .times(1)
            .returning(|_| Ok("rustc 0.9.0".to_string()));

        let checker = SystemRequirementsChecker {
            system: Box::new(mock_system),
        };
        let result = checker.check(requirements);

        assert!(result.is_err());
    }
}
