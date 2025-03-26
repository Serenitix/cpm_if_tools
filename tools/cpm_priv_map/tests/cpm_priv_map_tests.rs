use reqwest::blocking::get;
use serde_yaml;
use std::error::Error;
use cpm_priv_map::cpm_priv_map::{CPMPrivMap, CallRetPrivField, RWPrivField, ContextField};

fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    let content = response.text()?;
    Ok(content)
}

#[test]
fn test_load_sample_privilege_map() {
    let url = "https://raw.githubusercontent.com/ndauten/CPM-Interchange-Format/refs/heads/main/examples/linux_2.yaml";
    let yaml = download_file(url).unwrap();
    let privilege_map: CPMPrivMap = serde_yaml::from_str(&yaml).unwrap();
    
    // Add assertions to verify the loaded privilege map using the new public interfaces
    assert!(privilege_map.object_map().len() > 0);
    assert!(privilege_map.subject_map().len() > 0);
    assert!(privilege_map.privileges().len() > 0);

    // Additional assertions to verify the contents of the privilege map
    for privilege in privilege_map.privileges() {
        let principal = privilege.principal();
        let principal_subject = principal.subject();
        
        match principal.execution_context() {
            Some(ContextField::All) => (),
            Some(ContextField::Context(_)) => (),
            None => panic!("execution_context is None"),
        }

        match privilege.can_call() {
            Some(CallRetPrivField::All) => (),
            Some(CallRetPrivField::List(_)) => (),
            None => panic!("can_call is None"),
        }

        match privilege.can_return() {
            Some(CallRetPrivField::All) => (),
            Some(CallRetPrivField::List(_)) => (),
            None => panic!("can_return is None"),
        }

        match privilege.can_read() {
            Some(RWPrivField::All) => (),
            Some(RWPrivField::List(_)) => (),
            None => panic!("can_read is None"),
        }

        match privilege.can_write() {
            Some(RWPrivField::All) => (),
            Some(RWPrivField::List(_)) => (),
            None => panic!("can_write is None"),
        }
    }
}