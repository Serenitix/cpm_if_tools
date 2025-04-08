use serde_json;
use serde_yaml;
use jsonschema::{JSONSchema, ValidationError};
use std::fs;
use std::error::Error;

/// Validates a YAML file against a JSON schema.
///
/// # Arguments
/// * `schema_file` - Path to the JSON schema file.
/// * `yaml_file` - Path to the YAML file to validate.
///
/// # Returns
/// * `Ok(())` if the YAML file is valid.
/// * `Err(String)` if validation fails with a detailed error message.
pub fn validate_yaml(schema_file: &str, yaml_file: &str) -> Result<(), String> {
    // Read and parse the JSON schema
    let schema_content = fs::read_to_string(schema_file)
        .map_err(|e| format!("Failed to read schema file: {}", e))?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_content)
        .map_err(|e| format!("Failed to parse schema file as JSON: {}", e))?;

    // Read and parse the YAML file
    let yaml_content = fs::read_to_string(yaml_file)
        .map_err(|e| format!("Failed to read YAML file: {}", e))?;
    let yaml_data: serde_json::Value = serde_yaml::from_str(&yaml_content)
        .map_err(|e| format!("Failed to parse YAML file: {}", e))?;

    // Compile the schema
    let compiled_schema = JSONSchema::compile(&schema_json)
        .map_err(|e| format!("Failed to compile schema: {}", e))?;

    // Validate the YAML data against the schema
    if let Err(errors) = compiled_schema.validate(&yaml_data) {
        let error_messages: Vec<String> = errors
            .map(|e| e.to_string())
            .collect();
        return Err(format!(
            "Validation failed with the following errors:\n{}",
            error_messages.join("\n")
        ));
    }

    Ok(())
}