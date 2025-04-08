# CPM Schema Validator

## Overview
The **CPM Schema Validator** is a Rust-based tool designed to validate CPM policies as presented in YAML files against a **Compartmentalization Policy Model (CPM) schema**. This ensures that input files conform to the expected format and structure.

In addition to the command-line tool, this project provides a library that implements a custom definition of deep specification of the grammar, going beyond the syntax-only validation provided by the schema validator.

## Features
- Validates YAML data against the **CPM schema**.
- Provides a library for advanced grammar validation and manipulation.
- Includes **comprehensive unit tests** to verify correctness.
- Supports **command-line arguments** for flexibility.
- Provides a **Makefile** for simplified setup and usage.

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

### Using the Makefile
The project includes a `Makefile` to simplify common tasks. Below are the available commands:

- **Build the tool**:
  ```sh
  make build
  ```
  This runs `cargo build --release` to build the tool in release mode.

- **Run the validator**:
  ```sh
  make validate SCHEMA=<schema_file> INPUT=<yaml_file>
  ```
  This validates the specified YAML file against the schema. Replace `<schema_file>` and `<yaml_file>` with the paths to your schema and YAML files.

  Example:
  ```sh
  make validate SCHEMA=cpm_schema.json INPUT=input.yaml
  ```

- **Run tests**:
  ```sh
  make test
  ```
  This runs `cargo test` to execute the unit tests.

- **Clean build artifacts**:
  ```sh
  make clean
  ```
  This runs `cargo clean` to remove build artifacts.

### Running the Validator Directly
To validate a YAML file against the CPM schema without using the `Makefile`, run:
```sh
./target/release/cpm_if validate <schema_file> <yaml_file>
```
Where:
- `<schema_file>` is the schema file.
- `<yaml_file>` is the file to be validated.

### Example
```sh
./target/release/cpm_if validate cpm_schema.json input.yaml
```

### Running Tests
To run the built-in unit tests:
```sh
cargo test
```

## Library Usage
This project also provides a library (`cpm_if`) that implements a custom definition of deep specification of the grammar. While the schema validator focuses on syntax-only validation, the library enables advanced validation and manipulation of CPM privilege maps.

The library can be used to:
- Parse and manipulate CPM privilege maps.
- Perform advanced validation beyond what is possible with the schema validator.

### Example
Here is an example of how the library might be used in a Rust project:
```rust
use cpm_if::privilege_map::CPMPrivMap;

fn main() {
    let privilege_map = CPMPrivMap {
        object_map: vec![],
        subject_map: vec![],
        privileges: vec![],
    };

    // Perform advanced validation or manipulation here
    println!("Privilege map: {:?}", privilege_map);
}
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
```

## Roadmap

### 1. Integrate Deep Semantic Validator
- Extend the current syntax-only schema validator to include deep semantic validation.
- Use the library's advanced grammar specification to validate relationships and constraints within the CPM privilege map.
- Ensure that the semantic validator is seamlessly integrated into the `validate` command.

### 2. Upgrade to v1.4 Support
- Update the schema and library to support the new features introduced in CPM v1.4.
- Add tests to ensure backward compatibility with v1.3 while supporting v1.4.
- Provide clear documentation on the differences between v1.3 and v1.4.

### 3. Provide Examples of Analysis on the CPMPrivMap
- Add example scripts or tools that demonstrate how to analyze the CPM privilege map.
- Examples may include:
  - Identifying unused objects or subjects.
  - Detecting privilege escalation paths.
  - Visualizing relationships between objects, subjects, and privileges.
- Include these examples in the `examples/` directory for easy access.

## Contributing
Feel free to submit **pull requests** or **open issues** for bug reports or feature requests.

## License
This project is licensed under the **MIT License**.