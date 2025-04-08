# CPM Schema Validator

## Overview
The **CPM Schema Validator** is a Rust-based tool designed to validate CPM policies as presented in YAML files against a **Compartmentalization Policy Model (CPM) schema**. This ensures that input files conform to the expected format and structure.

## Features
- Validates YAML data against the **CPM schema**.
- Includes **comprehensive unit tests** to verify correctness.
- Supports **command-line arguments** for flexibility.

## Installation

The tool is written in Rust, making it the sole dependency.
Each of the following commands can be done in terminal or
the included Makefile will invoke each accordingly.

### Prerequisites
Ensure that you have **Rust** installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone the Repository
```sh
git clone <repository_url>
cd <repository_directory>
```

### Install Dependencies
```sh
cargo build --release
```

## Usage

### Running the Validator
To validate a YAML file against the CPM schema, run:
```sh
./target/release/schema_validator cpm_schema.json input.yaml
```
Where:
- `cpm_schema.json` is the schema file.
- `input.json` is the file to be validated.

### Running Tests
To run the built-in unit tests:
```sh
cargo test
```

## Schema Structure
The CPM schema consists of:
- **`object_map`**: Defines object domains.
- **`subject_map`**: Defines subject domains.
- **`privileges`**: Specifies access control rules.

## Example YAML Input
```yaml
object_map:
    - name: ObjectDomain1
    objects: ["object1", "object2"]
  
subject_map:
    - name: SubjectDomain1
    subjects: ["subject1"]
  
privileges:
    - principal:
    subject: SubjectDomain1
    execution_context:
      uid: user
    can_call: ["SubjectDomain2"]
    can_return: []
    can_read:
      - objects: ["ObjectDomain1"]
    can_write: []
  ]
}
```

## Contributing
Feel free to submit **pull requests** or **open issues** for bug reports or feature requests.

## License
This project is licensed under the **MIT License**.
