#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;
    use serde_json::json;
    use std::fs;
    use std::env;

    fn load_schema() -> serde_json::Value {
        let args: Vec<String> = env::args().collect();
        let schema_path = if args.len() > 1 {
            &args[1]
        } else {
            "tests/cpm_if_schema_v1.3.json"
        };
        let schema_content = fs::read_to_string(schema_path).expect("Failed to read schema file");
        serde_json::from_str(&schema_content).expect("Invalid JSON schema format")
    }

    #[test]
    fn test_valid_object_map() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let valid_data = json!({
            "object_map": [{ "name": "ObjectDomain1", "objects": ["object1", "object2"] }],
            "subject_map": [],
            "privileges": []
        });
        assert!(schema.validate(&valid_data).is_ok(), "Valid object_map should pass");
    }

    #[test]
    fn test_invalid_object_map() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let invalid_data = json!({
            "object_map": [{ "name": "ObjectDomain1" }],
            "subject_map": [],
            "privileges": []
        });
        assert!(schema.validate(&invalid_data).is_err(), "Missing 'objects' should fail");
    }

    #[test]
    fn test_valid_subject_map() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let valid_data = json!({
            "object_map": [],
            "subject_map": [{ "name": "SubjectDomain1", "subjects": ["subject1"] }],
            "privileges": []
        });
        assert!(schema.validate(&valid_data).is_ok(), "Valid subject_map should pass");
    }

    #[test]
    fn test_missing_required_fields() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let invalid_data = json!({
            "subject_map": [],
            "privileges": []
        });
        assert!(schema.validate(&invalid_data).is_err(), "Missing required 'object_map' should fail");
    }

    #[test]
    fn test_extra_unexpected_fields() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let invalid_data = json!({
            "object_map": [],
            "subject_map": [],
            "privileges": [],
            "unexpected_field": "some_value"
        });
        assert!(schema.validate(&invalid_data).is_err(), "Unexpected fields should fail");
    }

    #[test]
    fn test_invalid_privileges_format() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let invalid_data = json!({
            "object_map": [],
            "subject_map": [],
            "privileges": [{
                "principal": {
                    "subject": "SubjectDomain1"
                },
                "can_call": "SubjectDomain2"
            }]
        });
        assert!(schema.validate(&invalid_data).is_err(), "Incorrect format in 'can_call' should fail");
    }
}
