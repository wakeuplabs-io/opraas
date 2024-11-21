use serde_yaml::{Value, to_writer};
use std::fs::File;
use std::collections::HashMap;

pub fn rewrite_yaml_to(from: &str, to: &str, updates: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(from)?;
    let mut yaml: Value = serde_yaml::from_reader(file)?;

    // Loop through all the updates and apply them
    for (key, new_value) in updates {
        // Split the key by '.' to access nested fields
        let keys: Vec<&str> = key.split('.').collect();

        // Update the value in the nested structure
        let mut current = &mut yaml;
        for (i, key_part) in keys.iter().enumerate() {
            // If we're at the last part of the key, update the value
            if i == keys.len() - 1 {
                if let Value::Mapping(map) = current {
                    map.insert(Value::String(key_part.to_string()), Value::String(new_value.to_string()));
                }
            } else {
                // Otherwise, move deeper into the structure
                if let Value::Mapping(map) = current {
                    current = map.entry(Value::String(key_part.to_string()))
                        .or_insert(Value::Mapping(serde_yaml::Mapping::new()));
                }
            }
        }
    }

    // Write the updated YAML back to the file
    let mut file = File::create(to)?;
    to_writer(&mut file, &yaml)?;

    Ok(())
}