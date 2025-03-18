use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::process;
use crate::object_identifer::ObjectIdentifier;

#[derive(Debug, Deserialize, Serialize)]
pub struct CPMPrivMap {
    object_map: Vec<ObjectDomain>,
    subject_map: Vec<SubjectDomain>,
    privileges: Vec<Privilege>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ObjectDomain {
    name: String,
    objects: Vec<ObjectIdentifier>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubjectDomain {
    name: String,
    subjects: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Privilege {
    principal: Principal,
    can_call: Option<Vec<String>>,
    can_return: Option<Vec<String>>,
    can_read: Option<Vec<AccessDescriptor>>,
    can_write: Option<Vec<AccessDescriptor>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Principal {
    subject: String,
    execution_context: Option<ExecutionContext>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExecutionContext {
    call_context: Option<Vec<String>>,
    uid: Option<String>,
    gid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AccessDescriptor {
    objects: Vec<ObjectIdentifier>,
    object_context: Option<ExecutionContext>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cpm_yaml_validator <file.yaml>");
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read YAML file");
    
    match serde_yaml::from_str::<CPMPrivMap>(&content) {
        Ok(_) => println!("{} is valid according to the CPM schema.", filename),
        Err(e) => {
            eprintln!("Validation error: {}", e);
            process::exit(1);
        }
    }
}
