#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;
    use serde_json::json;
    use std::fs;

    fn load_schema() -> serde_json::Value {
        //-- the test schema is referenced from where the test is run --/
        let schema_content = fs::read_to_string("tests/cpm_if_schema_v1.3.json").expect("Failed to read schema file");
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
    fn test_valid_privileges() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let valid_data = json!({
            "object_map": [],
            "subject_map": [],
            "privileges": [{
                "principal": {
                    "subject": "SubjectDomain1",
                    "execution_context": { "uid": "user" }
                },
                "can_call": ["SubjectDomain2"],
                "can_return": [],
                "can_read": [{ "objects": ["ObjectDomain1"] }],
                "can_write": []
            }]
        });
        assert!(schema.validate(&valid_data).is_ok(), "Valid privileges should pass");
    }

    #[test]
    fn test_invalid_privileges() {
        let schema = JSONSchema::compile(&load_schema()).expect("Failed to compile schema");
        let invalid_data = json!({
            "object_map": [],
            "subject_map": [],
            "privileges": [{
                "principal": {
                    "execution_context": { "uid": "user" }
                }
            }]
        });
        assert!(schema.validate(&invalid_data).is_err(), "Missing subject in principal should fail");
    }
}
