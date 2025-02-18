// SPDX-License-Identifier: MIT
// 
// MIT License
// 
// © 2024 Nathan Dautenhahn & Serenitix LLC
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use serde::{Deserialize, Serialize};
use serde_json;
use jsonschema::{JSONSchema};
use std::fs;
use std::process;

//--- Struct for typing the serialized schema ---//
#[derive(Debug, Deserialize, Serialize)]
struct JsonSchema {
    #[serde(rename = "$schema")]
    schema: String,
    properties: serde_json::Value,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cpm_yaml_validator <schema.json> <file.yaml>");
        process::exit(1);
    }

    let schema_file = &args[1];
    let yaml_file = &args[2];

    let schema_content = fs::read_to_string(schema_file).expect("Failed to read JSON schema file");
    let yaml_content = fs::read_to_string(yaml_file).expect("Failed to read YAML file");
    
    //--- import json into serde_json object ---// 
    let schema_json: serde_json::Value = match serde_json::from_str(&schema_content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Schema parsing error: {}", e);
            process::exit(1);
        }
    };
    
    //--- import the IF formatted policy ---//
    let yaml_data: serde_json::Value = match serde_yaml::from_str(&yaml_content) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("YAML parsing error: {}", e);
            process::exit(1);
        }
    };

    //--- compile the schema into the validator object to be used for comparing ---//
    let compiled_schema = match JSONSchema::compile(&schema_json) {
        Ok(schema) => schema,
        Err(e) => {
            eprintln!("Failed to compile JSON Schema: {}", e);
            process::exit(1);
        }
    };

    //--- using the schema as defined in the provided json file validate the IF yaml ---//
    match compiled_schema.validate(&yaml_data) {
        Ok(_) => println!("YAML file is valid according to the schema."),
        Err(errors) => {
            eprintln!("YAML validation errors:");
            for error in errors {
                eprintln!("{}", error);
            }
            process::exit(1);
        }
    };
}
