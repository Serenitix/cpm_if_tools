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

use cpm_if::validate_yaml::{validate_yaml};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cpm_if validate <schema.json> <file.yaml>");
        process::exit(1);
    }

    let command = &args[1];
    let schema_file = &args[2];
    let yaml_file = &args[3];

    match command.as_str() {
        "validate" => {
            if let Err(e) = validate_yaml(schema_file, yaml_file) {
                eprintln!("Validation failed: {}", e);
                process::exit(1);
            } else {
                println!("Validation succeeded!");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: cpm_if validate <schema.json> <file.yaml>");
            process::exit(1);
        }
    }
}