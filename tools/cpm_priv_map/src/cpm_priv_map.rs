use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{self, Visitor};
use serde::ser::Serializer;
use std::char::ToUppercase;
use std::fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

//pub mod object_id;

//use object_identifer::ObjectID;
// TODO make sure to specify yaml serialization and deserialization

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CPMPrivMap {
    pub object_map: Vec<ObjectDomain>,
    pub subject_map: Vec<SubjectDomain>,
    pub privileges: Vec<Privilege>,
}

impl CPMPrivMap {

    pub fn new() -> Self {
        Self {
            object_map: vec![],
            subject_map: vec![],
            privileges: vec![],
        }
    }

    pub fn object_map(&self) -> &Vec<ObjectDomain> {
        &self.object_map
    }

    pub fn add_object_domain(&mut self, object_domain: ObjectDomain) {
        self.object_map.push(object_domain);
    }

    pub fn subject_map(&self) -> &Vec<SubjectDomain> {
        &self.subject_map
    }

    pub fn add_subject_domain(&mut self, subject_domain: SubjectDomain) {
        self.subject_map.push(subject_domain);
    }

    pub fn privileges(&self) -> &Vec<Privilege> {
        &self.privileges
    }

    pub fn save_to_yaml(&self, file_path: &str) -> 
        Result<(), Box<dyn std::error::Error>> 
    {
        // Serialize the CPMPrivMap to a YAML string
        let serialized_yaml = serde_yaml::to_string(self)?;
        
        // Create or overwrite the file
        let mut file = File::create(file_path)?;
        
        // Write the serialized YAML to the file
        file.write_all(serialized_yaml.as_bytes())?;
        
        Ok(())
    }

}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ObjectDomain {
    name: String,
    //objects: Vec<String>,
    objects: Vec<ObjectID>,
}

impl ObjectDomain {

    pub fn new(name: String, objects: Vec<ObjectID>) -> Self {
        Self { name, objects }
    }
    pub fn new_empty(name: String) -> Self {
        Self { name, objects: vec![] }
    }
    pub fn add_object(&mut self, object: ObjectID) {
        self.objects.push(object);
    }
    pub fn remove_object(&mut self, object: &ObjectID) {
        self.objects.retain(|o| o != object);
    }
    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }
    pub fn set_objects(&mut self, objects: Vec<ObjectID>) {
        self.objects = objects;
    }
    pub fn get_object(&self, index: usize) -> Option<&ObjectID> {
        self.objects.get(index)
    }
    pub fn get_object_by_name(&self, name: &str) -> Option<&ObjectID> {
        self.objects.iter().find(|o| o.name == name)
    }
    pub fn get_object_by_path(&self, path: &str) -> Option<&ObjectID> {
        self.objects.iter().find(|o| o.path == path)
    }
    pub fn get_object_by_lineno(&self, lineno: &str) -> Option<&ObjectID> {
        self.objects.iter().find(|o| o.lineno == lineno)
    }
    pub fn get_object_by_alloc_type(&self, alloc_type: &AllocType) -> Option<&ObjectID> {
        self.objects.iter().find(|o| o.alloc_type == *alloc_type)
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn objects(&self) -> &Vec<ObjectID> {
        &self.objects
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectID {
    alloc_type: AllocType,
    path: String,
    lineno: String,
    name: String,
}

impl ObjectID {
    pub fn new(alloc_type: AllocType, path: String, lineno: String, name: String) -> Self {
        Self {
            alloc_type,
            path,
            lineno,
            name,
        }
    }

    pub fn alloc_type(&self) -> &AllocType {
        &self.alloc_type
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn lineno(&self) -> &str {
        &self.lineno
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

// Grammar: "<alloc_type>|<path>|<lineno>|<name>"
impl Serialize for ObjectID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!(
            "{}|{}|{}|{}",
            self.alloc_type, self.path, self.lineno, self.name
        );
        serializer.serialize_str(&s)
    }
}

// Grammar: "<alloc_type>|<path>|<lineno>|<name>"
impl<'de> Deserialize<'de> for ObjectID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('|').collect();

        if parts.len() == 4 {
            // If the format is correct, parse the fields
            let alloc_type = parts[0].parse::<AllocType>().map_err(de::Error::custom)?;
            Ok(ObjectID {
                alloc_type,
                path: parts[1].to_string(),
                lineno: parts[2].to_string(),
                name: parts[3].to_string(),
            })
        } else {
            // Fallback: Treat the entire string as the `name` field
            Ok(ObjectID {
                alloc_type: AllocType::Other, // Default alloc_type
                path: "".to_string(),         // Default empty path
                lineno: "".to_string(),       // Default empty line number
                name: s,                      // Use the entire string as the name
            })
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")] // Automatically convert to uppercase
pub enum AllocType {
    Global,
    Local,
    Heap,
    StackFrame,
    StackRegion,
    IO,
    Other
}

impl fmt::Display for AllocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocType::Global => write!(f, "GLOBAL"),
            AllocType::Local => write!(f, "LOCAL"),
            AllocType::Heap => write!(f, "HEAP"),
            AllocType::StackFrame => write!(f, "STACK_FRAME"),
            AllocType::StackRegion => write!(f, "STACK_REGION"),
            AllocType::IO => write!(f, "IO"),
            AllocType::Other => write!(f, "OTHER"),
        }
    }
}

const ALLOWED_ALLOCATORS: &[&str] = &[
    "kmalloc_reserve",
    "xdr_alloc_bvec",
    "__netdev_alloc_skb",
    "kmemdup_nul",
    "dst_cow_metrics_generic",
    "nfs_writehdr_alloc",
    "rpc_malloc",
    "unx_lookup_cred",
    "xprt_alloc_slot",
    "nfs_page_create",
    "nfs_readhdr_alloc",
    "dst_alloc",
    "rpc_new_task",
    "___neigh_create",
    "__alloc_skb",
];

impl std::str::FromStr for AllocType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GLOBAL" => Ok(AllocType::Global),
            "LOCAL" => Ok(AllocType::Local),
            "HEAP" => Ok(AllocType::Heap),
            "STACK_FRAME" => Ok(AllocType::StackFrame),
            "STACK_REGION" => Ok(AllocType::StackRegion),
            "IO" => Ok(AllocType::IO),
            "OTHER" => Ok(AllocType::Other),
            _ => {
                if ALLOWED_ALLOCATORS.contains(&s) {
                    Ok(AllocType::Heap)
                } else {
                    // Print a warning to stderr for unknown allocators
                    eprintln!("Warning: Unknown allocator '{}', defaulting to OTHER", s);
                    Ok(AllocType::Other)
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubjectDomain {
    name: String,
    subjects: Vec<String>,
}

impl SubjectDomain {

    pub fn new(name: String, subjects: Vec<String>) -> Self {
        Self { name, subjects }
    }

    pub fn add_subject(&mut self, subject: String) {
        self.subjects.push(subject);
    }

    pub fn add_subjects(&mut self, subjects: Vec<String>) {
        self.subjects.extend(subjects);
    }

    pub fn new_empty(name: String) -> Self {
        Self { name, subjects: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subjects(&self) -> &Vec<String> {
        &self.subjects
    }
}

/*
 * Grammar: 
 *      Privilege ::= { 
 *          principal: Principal,
 *          ? can_call: [ SubjectDomainName ] | all,
 *          ? can_return: [ SubjectDomainName ] | all,
 *          ? can_read: [ Object ] | all,
 *          ? can_write: [ Object ] | all,
 *      } 
 */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Privilege {
    pub principal: Principal,
    #[serde(default = "default_callret_priv_field")]
    pub can_call: CallRetPrivField,    
    #[serde(default = "default_callret_priv_field")]
    pub can_return: CallRetPrivField,
    #[serde(default = "default_rw_priv_field")]
    pub can_read: RWPrivField,
    #[serde(default = "default_rw_priv_field")]
    pub can_write: RWPrivField,
}

impl Privilege {
    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    pub fn can_call(&self) -> &CallRetPrivField {
        &self.can_call
    }

    pub fn can_return(&self) -> &CallRetPrivField {
        &self.can_return
    }

    pub fn can_read(&self) -> &RWPrivField {
        &self.can_read
    }

    pub fn can_write(&self) -> &RWPrivField {
        &self.can_write
    }
}

fn default_callret_priv_field() -> CallRetPrivField {
    CallRetPrivField::All
}

#[derive(Debug, PartialEq)]
pub enum CallRetPrivField {
    // TODO: switch to ObjectIdentifier/SubjectIdentifiers
    // Grammar: ? can call: [ SubjectDomainName ] | all,
    List(Vec<String>),
    All,
}

/* Default all non vector values to All */
impl<'de> Deserialize<'de> for CallRetPrivField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallRetPrivFieldVisitor;

        impl<'de> Visitor<'de> for CallRetPrivFieldVisitor {
            type Value = CallRetPrivField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of strings or the string \"all\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "all" {
                    Ok(CallRetPrivField::All)
                } else {
                    Err(de::Error::unknown_variant(value, &["all"]))
                }
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let values = Vec::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                Ok(CallRetPrivField::List(values))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Handle explicitly empty fields (e.g., `can_call:`)
                Ok(CallRetPrivField::All)
            }

        }

        deserializer.deserialize_any(CallRetPrivFieldVisitor)
    }
}

impl Serialize for CallRetPrivField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CallRetPrivField::List(calls) => calls.serialize(serializer), // Serialize as a plain list
            CallRetPrivField::All => serializer.serialize_str("all"),    // Serialize "All" as a string
        }
    }
}

fn default_rw_priv_field() -> RWPrivField {
    RWPrivField::All
}

#[derive(Debug, Serialize, PartialEq)]
pub enum RWPrivField {
    List(Vec<Object>),
    #[serde(rename = "all")] // Serialize/deserialize "All" as "all"
    All,
}

impl<'de> Deserialize<'de> for RWPrivField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RWPrivFieldVisitor;

        impl<'de> Visitor<'de> for RWPrivFieldVisitor {
            type Value = RWPrivField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of strings or the string \"all\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "all" {
                    Ok(RWPrivField::All)
                } else {
                    Err(de::Error::unknown_variant(value, &["all"]))
                }
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let values = Vec::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                Ok(RWPrivField::List(values))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Handle explicitly empty fields (e.g., `can_call:`)
                Ok(RWPrivField::All)
            }

        }

        deserializer.deserialize_any(RWPrivFieldVisitor)
    }
}

/*
 * Principal ::= { subject: SubjectDomain, ? execution context: Context | all }
 *   - if field missing, default to all, if it is then parse to all or Context
 */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Principal {
    // TODO make this work correctly: point to a subject domain 
    //   eg: subject: SubjectDomain, // but a reference to a subject domain
    //   well the grammar specifies it like this right now. so i will be 
    //   faithful to the grammar and propose changes after one version
    //subject: SubjectDomain,
    pub subject: String,
    #[serde(default = "default_context_field")]
    pub execution_context: ContextField,
}

impl Principal {
    /*
    TODO Make proper subject doamin in next iteration, string for now
    pub fn subject(&self) -> &SubjectDomain {
        &self.subject
    }
    */
    pub fn subject(&self) -> &String {
        &self.subject
    }

    pub fn execution_context(&self) -> &ContextField {
        &self.execution_context
    }
}

fn default_context_field() -> ContextField {
    ContextField::All
}

/*
 * The context field is used for execution_context and object_context fields 
 * of the Principal and Object objects. The logic of the grammar allows for an 
 * optional context field that is either non-existent and defaults to "all" or 
 * as a context object. This enum allows for either a defined context or "all", 
 * which then leads to simpler serialization and deserialization.
 */
#[derive(Debug, Serialize, PartialEq)]
pub enum ContextField {
    Context(Context),
    #[serde(rename = "all")] // Serialize/deserialize "All" as "all"
    All,
}

/*
 * The field may be: missing, "all", or a context object. 
 * Otherwise, match either the string "all" or a Context object. 
 */
impl<'de> Deserialize<'de> for ContextField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextFieldVisitor;

        impl<'de> Visitor<'de> for ContextFieldVisitor {
            type Value = ContextField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a context object or the string \"all\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "all" {
                    Ok(ContextField::All)
                } else {
                    Err(de::Error::unknown_variant(value, &["all"]))
                }
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let context = Context::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ContextField::Context(context))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ContextField::All)
            }
        }

        deserializer.deserialize_any(ContextFieldVisitor)
    }
}

// Context ::= { ? call_context: [ SubjectDomainName | all ],
//               ? uid: root | user | Variable | all,
//               ? guid: Variable | all }
// TODO: handle the option and default values correctly
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Context {
    #[serde(default = "default_call_context_sub_field")]
    call_context: Option<CallContextSubField>,
    #[serde(default = "default_context_simple")]
    uid: Option<ContextSimpleString>,
    #[serde(default = "default_context_simple")]
    gid: Option<ContextSimpleString>,
}

impl Context {
    pub fn call_context(&self) -> &Option<CallContextSubField> {
        &self.call_context
    }

    pub fn uid(&self) -> &Option<ContextSimpleString> {
        &self.uid
    }

    pub fn gid(&self) -> &Option<ContextSimpleString> {
        &self.gid
    }
}

fn default_call_context_sub_field() -> Option<CallContextSubField> {
    Some(CallContextSubField::All) // Placeholder for yet to be implemented
}

#[derive(Debug, PartialEq)]
/*
 * This serializes to a vector of strings or a vector of a single string "all"
 */
pub enum CallContextSubField {
    List(Vec<String>),
    All,
}

/* Only implement two variants and leave the complex one for later */
impl Serialize for CallContextSubField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            CallContextSubField::List(ref values) => values.serialize(serializer),
            CallContextSubField::All => vec!["all".to_string()].serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CallContextSubField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallContextSubFieldVisitor;

        impl<'de> Visitor<'de> for CallContextSubFieldVisitor {
            type Value = CallContextSubField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of strings or a list with a single string \"all\"")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let values: Vec<String> = Vec::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                if values.len() == 1 && values[0] == "all" {
                    Ok(CallContextSubField::All)
                } else {
                    Ok(CallContextSubField::List(values))
                }
            }
        }

        deserializer.deserialize_any(CallContextSubFieldVisitor)
    }
}

fn default_context_simple() -> Option<ContextSimpleString> {
    Some(ContextSimpleString::All) 
}

#[derive(Debug, PartialEq)]
pub enum ContextSimpleString {
    String(String),
    All,
}

impl Serialize for ContextSimpleString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            ContextSimpleString::String(ref value) => value.serialize(serializer),
            ContextSimpleString::All => "all".serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ContextSimpleString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextSimpleStringVisitor;

        impl<'de> Visitor<'de> for ContextSimpleStringVisitor {
            type Value = ContextSimpleString;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or the string \"all\"")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "all" {
                    Ok(ContextSimpleString::All)
                } else {
                    Ok(ContextSimpleString::String(value.to_string()))
                }
            }
        }

        deserializer.deserialize_str(ContextSimpleStringVisitor)
    }
}

/*
 * Grammar: Object ::= { objects: [ ObjectDomainName ] | all
 *                     ? object_context: Context | all }
 */
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Object {
    objects: Vec<String>,
    ///objects: Vec<ObjectIdentifier>,
    #[serde(default = "default_context_field")]
    object_context: ContextField,
}

impl Object {
    pub fn objects(&self) -> &Vec<String> {
        &self.objects
    }

    pub fn object_context(&self) -> &ContextField {
        &self.object_context
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_empty_execution_context() {
        let yaml = r#"
principal:
subject: main_domain
execution_context:
    "#;
        let result: Principal = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            result,
            Principal {
                subject: "main_domain".to_string(),
                execution_context: ContextField::All, // Expecting default to "All"
            }
        );
    }
    
    #[test]
    fn test_deserialize_empty_object_context() {
        let yaml = r#"
objects:
- object1
- object2
object_context:
    "#;
        let result: Object = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            result,
            Object {
                objects: vec!["object1".to_string(), "object2".to_string()],
                object_context: ContextField::All, // Expecting default to "All"
            }
        );
    }

    #[test]
    fn test_deserialize_callret_priv_field_all() {
        let yaml = "all";
        let result: CallRetPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallRetPrivField::All);
    }

    #[test]
    fn test_deserialize_callret_priv_field_list() {
        let yaml = "- domain1\n- domain2";
        let result: CallRetPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallRetPrivField::List(vec!["domain1".to_string(), "domain2".to_string()]));
    }

    #[test]
    fn test_deserialize_callret_priv_field_empty_list() {
        let yaml = "[]";
        let result: CallRetPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallRetPrivField::List(vec![]));
    }

    #[test]
    fn test_deserialize_rw_priv_field_all() {
        let yaml = "all";
        let result: RWPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, RWPrivField::All);
    }

    #[test]
    fn test_deserialize_rw_priv_field_list() {
        let yaml = "- objects:\n  - object1\n  - object2";
        let result: RWPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, RWPrivField::List(vec![Object {
            objects: vec!["object1".to_string(), "object2".to_string()],
            object_context: ContextField::All,
        }]));
    }

    #[test]
    fn test_deserialize_rw_priv_field_empty_list() {
        let yaml = "[]";
        let result: RWPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, RWPrivField::List(vec![]));
    }

    #[test]
    fn test_deserialize_call_context_sub_field_all() {
        let yaml = "- all";
        let result: CallContextSubField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallContextSubField::All);
    }

    #[test]
    fn test_deserialize_call_context_sub_field_list() {
        let yaml = "- domain1\n- domain2";
        let result: CallContextSubField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallContextSubField::List(vec!["domain1".to_string(), "domain2".to_string()]));
    }

    #[test]
    fn test_deserialize_call_context_sub_field_empty_list() {
        let yaml = "[]";
        let result: CallContextSubField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, CallContextSubField::List(vec![]));
    }

    #[test]
    fn test_serialize_call_context_sub_field_all() {
        let value = CallContextSubField::All;
        let yaml = serde_yaml::to_string(&value).unwrap();
        assert_eq!(yaml, "- all\n");
    }

    #[test]
    fn test_serialize_call_context_sub_field_list() {
        let value = CallContextSubField::List(vec!["domain1".to_string(), "domain2".to_string()]);
        let yaml = serde_yaml::to_string(&value).unwrap();
        assert_eq!(yaml, "- domain1\n- domain2\n");
    }

    #[test]
    fn test_deserialize_context_field_all() {
        let yaml = "all";
        let result: ContextField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, ContextField::All);
    }

    #[test]
    fn test_deserialize_context_field_context() {
        let yaml = "call_context:\n  - domain1\n  - domain2\nuid: root\ngid: group1";
        let result: ContextField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, ContextField::Context(Context {
            call_context: Some(CallContextSubField::List(vec!["domain1".to_string(), "domain2".to_string()])),
            uid: Some(ContextSimpleString::String("root".to_string())),
            gid: Some(ContextSimpleString::String("group1".to_string())),
        }));
    }

    #[test]
    fn test_deserialize_context_field_missing_fields() {
        let yaml = "call_context:\n  - domain1\n  - domain2";
        let result: ContextField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, ContextField::Context(Context {
            call_context: Some(CallContextSubField::List(vec!["domain1".to_string(), "domain2".to_string()])),
            uid: Some(ContextSimpleString::All),
            gid: Some(ContextSimpleString::All),
        }));
    }

    #[test]
    fn test_serialize_object_id() {
        let object_id = ObjectID {
            alloc_type: AllocType::Global,
            path: "/path/to/file".to_string(),
            lineno: "42".to_string(),
            name: "my_object".to_string(),
        };

        let serialized = serde_json::to_string(&object_id).unwrap();
        assert_eq!(serialized, "\"GLOBAL|/path/to/file|42|my_object\"");
    }

    #[test]
    fn test_deserialize_object_id() {
        let serialized = "\"GLOBAL|/path/to/file|42|my_object\"";

        let deserialized: ObjectID = serde_json::from_str(serialized).unwrap();
        assert_eq!(
            deserialized,
            ObjectID {
                alloc_type: AllocType::Global,
                path: "/path/to/file".to_string(),
                lineno: "42".to_string(),
                name: "my_object".to_string(),
            }
        );
    }

    /* TODO ADD LATER
    #[test]
    fn test_invalid_alloc_type() {
        let serialized = "\"Invalid|/path/to/file|42|my_object\"";

        let deserialized: Result<ObjectID, _> = serde_json::from_str(serialized);
        assert!(deserialized.is_err());
    }
    */

    #[test]
    fn test_unknown_allocator_warning() {
        let allocators = vec![
            "kmalloc_reserve", // Known allocator
            "unknown_allocator_1", // Unknown allocator
            "xdr_alloc_bvec", // Known allocator
            "unknown_allocator_2", // Unknown allocator
        ];
    
        for allocator in allocators {
            let alloc_type: AllocType = allocator.parse().unwrap();
            if allocator.starts_with("unknown") {
                assert_eq!(alloc_type, AllocType::Other);
            }
        }
    }

    #[test]
    fn test_alloc_type_to_string() {
        assert_eq!(AllocType::Global.to_string(), "GLOBAL");
        assert_eq!(AllocType::Local.to_string(), "LOCAL");
        assert_eq!(AllocType::Heap.to_string(), "HEAP");
        assert_eq!(AllocType::StackFrame.to_string(), "STACK_FRAME");
        assert_eq!(AllocType::StackRegion.to_string(), "STACK_REGION");
        assert_eq!(AllocType::IO.to_string(), "IO");
        assert_eq!(AllocType::Other.to_string(), "OTHER");
    }

    #[test]
    fn test_alloc_type_from_str() {
        assert_eq!("GLOBAL".parse::<AllocType>().unwrap(), AllocType::Global);
        assert_eq!("LOCAL".parse::<AllocType>().unwrap(), AllocType::Local);
        assert_eq!("HEAP".parse::<AllocType>().unwrap(), AllocType::Heap);
        assert_eq!("STACK_FRAME".parse::<AllocType>().unwrap(), AllocType::StackFrame);
        assert_eq!("STACK_REGION".parse::<AllocType>().unwrap(), AllocType::StackRegion);
        assert_eq!("IO".parse::<AllocType>().unwrap(), AllocType::IO);
        assert_eq!("OTHER".parse::<AllocType>().unwrap(), AllocType::Other);

        //let invalid = "Invalid".parse::<AllocType>();
        //assert!(invalid.is_err());
    }

    #[test]
    fn test_deserialize_invalid_data() {
        let yaml = "invalid_data";
        let result: Result<CallContextSubField, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_full_cpm_priv_map() {
        let yaml = "
    object_map:
      - name: domain1
        objects: [object1, object2]
    subject_map:
      - name: subject1
        subjects: [subject1, subject2]
    privileges:
      - principal:
          subject: subject1
          execution_context: all
        can_call: all
        can_return: all
        can_read: all
        can_write: all
    ";
        let result: CPMPrivMap = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            result,
            CPMPrivMap {
                object_map: vec![ObjectDomain::new(
                    "domain1".to_string(),
                    vec![
                        ObjectID::new(
                            AllocType::Other, // Default alloc_type for fallback
                            "".to_string(),   // Default empty path
                            "".to_string(),   // Default empty line number
                            "object1".to_string()
                        ),
                        ObjectID::new(
                            AllocType::Other, // Default alloc_type for fallback
                            "".to_string(),   // Default empty path
                            "".to_string(),   // Default empty line number
                            "object2".to_string()
                        ),
                    ]
                )],
                subject_map: vec![SubjectDomain {
                    name: "subject1".to_string(),
                    subjects: vec!["subject1".to_string(), "subject2".to_string()],
                }],
                privileges: vec![Privilege {
                    principal: Principal {
                        subject: "subject1".to_string(),
                        execution_context: ContextField::All,
                    },
                    can_call: CallRetPrivField::All,
                    can_return: CallRetPrivField::All,
                    can_read: RWPrivField::All,
                    can_write: RWPrivField::All,
                }],
            }
        );
    }

    #[test]
    fn test_serialize_deserialize_cpm_priv_map() {
        let input_yaml = r#"
object_map:
  - name: domain1
    objects: [object1, object2]
subject_map:
  - name: subject1
    subjects: [subject1, subject2]
privileges:
  - principal:
      subject: subject1
      execution_context: all
    can_call: all
    can_return: all
    can_read: all
    can_write: all
    "#;

        // Deserialize the YAML into a CPMPrivMap
        let deserialized: CPMPrivMap = serde_yaml::from_str(input_yaml).unwrap();

        // Serialize the CPMPrivMap back into YAML
        let serialized_yaml = serde_yaml::to_string(&deserialized).unwrap();

        // Deserialize the serialized YAML again to ensure consistency
        let re_deserialized: CPMPrivMap = serde_yaml::from_str(&serialized_yaml).unwrap();

        // Assert that the re-deserialized object matches the original deserialized object
        assert_eq!(deserialized, re_deserialized);

        // Optionally, check if the serialized YAML matches the input YAML (ignoring formatting differences)
        let normalized_input: CPMPrivMap = serde_yaml::from_str(input_yaml).unwrap();
        let normalized_serialized: CPMPrivMap = serde_yaml::from_str(&serialized_yaml).unwrap();
        assert_eq!(normalized_input, normalized_serialized);
    }

    #[test]
    fn test_save_to_yaml() {
        let mut cpm_pmap = CPMPrivMap::new();

        // Populate the CPMPrivMap with example data
        cpm_pmap.object_map.push(ObjectDomain::new(
            "domain1".to_string(),
            vec![
                ObjectID::new(
                    AllocType::Global,
                    "/path/to/file".to_string(),
                    "42".to_string(),
                    "object1".to_string(),
                ),
            ],
        ));

        // Save to a temporary file
        let file_path = "test_output.yaml";
        cpm_pmap.save_to_yaml(file_path).unwrap();

        // Read the file back and verify its contents
        let saved_yaml = std::fs::read_to_string(file_path).unwrap();
        let deserialized: CPMPrivMap = serde_yaml::from_str(&saved_yaml).unwrap();

        assert_eq!(cpm_pmap, deserialized);

        // Clean up the test file
        std::fs::remove_file(file_path).unwrap();
    }
}