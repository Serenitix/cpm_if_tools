use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{self, Visitor};
use std::fmt;
//use crate::object_identifer::ObjectID;
// TODO make sure to specify yaml serialization and deserialization

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CPMPrivMap {
    object_map: Vec<ObjectDomain>,
    subject_map: Vec<SubjectDomain>,
    privileges: Vec<Privilege>,
}

impl CPMPrivMap {
    pub fn object_map(&self) -> &Vec<ObjectDomain> {
        &self.object_map
    }

    pub fn subject_map(&self) -> &Vec<SubjectDomain> {
        &self.subject_map
    }

    pub fn privileges(&self) -> &Vec<Privilege> {
        &self.privileges
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ObjectDomain {
    name: String,
    objects: Vec<String>,
    //objects: Vec<ObjectID>,
}

impl ObjectDomain {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn objects(&self) -> &Vec<String> {
        &self.objects
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubjectDomain {
    name: String,
    subjects: Vec<String>,
}

impl SubjectDomain {
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
    principal: Principal,
    #[serde(default = "default_callret_priv_field")]
    can_call: CallRetPrivField,    
    #[serde(default = "default_callret_priv_field")]
    can_return: CallRetPrivField,
    #[serde(default = "default_rw_priv_field")]
    can_read: RWPrivField,
    #[serde(default = "default_rw_priv_field")]
    can_write: RWPrivField,
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

#[derive(Debug, Serialize, PartialEq)]
pub enum CallRetPrivField {
    // TODO: switch to ObjectIdentifier/SubjectIdentifiers
    // Grammar: ? can call: [ SubjectDomainName ] | all,
    List(Vec<String>),
    #[serde(rename = "all")] // Serialize/deserialize "All" as "all"
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
    subject: String,
    #[serde(default = "default_context_field")]
    execution_context: ContextField,
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
        assert_eq!(result, CPMPrivMap {
            object_map: vec![ObjectDomain {
                name: "domain1".to_string(),
                objects: vec!["object1".to_string(), "object2".to_string()],
            }],
            subject_map: vec![SubjectDomain {
                name: "subject1".to_string(),
                subjects: vec!["subject1".to_string(), "subject2".to_string()],
            }],
            privileges: vec![Privilege {
                principal: Principal {
                    /* TODO: switch to an actual object
                    subject: SubjectDomain {
                        name: "subject1".to_string(),
                        subjects: vec!["subject1".to_string(), "subject2".to_string()],
                    }, 
                    */
                    subject: "subject1".to_string(),
                    execution_context: ContextField::All,
                },
                can_call: CallRetPrivField::All,
                can_return: CallRetPrivField::All,
                can_read: RWPrivField::All,
                can_write: RWPrivField::All,
            }],
        });
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
}