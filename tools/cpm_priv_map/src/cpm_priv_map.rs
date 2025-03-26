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
    can_call: Option<CallRetPrivField>,    
    #[serde(default = "default_callret_priv_field")]
    can_return: Option<CallRetPrivField>,
    #[serde(default = "default_rw_priv_field")]
    can_read: Option<RWPrivField>,
    #[serde(default = "default_rw_priv_field")]
    can_write: Option<RWPrivField>,
}

impl Privilege {
    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    pub fn can_call(&self) -> &Option<CallRetPrivField> {
        &self.can_call
    }

    pub fn can_return(&self) -> &Option<CallRetPrivField> {
        &self.can_return
    }

    pub fn can_read(&self) -> &Option<RWPrivField> {
        &self.can_read
    }

    pub fn can_write(&self) -> &Option<RWPrivField> {
        &self.can_write
    }
}

fn default_callret_priv_field() -> Option<CallRetPrivField> {
    Some(CallRetPrivField::All)
}

#[derive(Debug, Serialize, PartialEq)]
pub enum CallRetPrivField {
    // TODO: switch to ObjectIdentifier/SubjectIdentifiers
    // Grammar: ? can call: [ SubjectDomainName ] | all,
    List(Vec<String>),
    All,
}

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
        }

        deserializer.deserialize_any(CallRetPrivFieldVisitor)
    }
}

fn default_rw_priv_field() -> Option<RWPrivField> {
    Some(RWPrivField::All)
}

#[derive(Debug, Serialize, PartialEq)]
pub enum RWPrivField {
    List(Vec<Object>),
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
                formatter.write_str("a list of objects or the string \"all\"")
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
    subject: SubjectDomain,
    #[serde(default = "default_context_field")]
    execution_context: Option<ContextField>,
}

impl Principal {
    pub fn subject(&self) -> &SubjectDomain {
        &self.subject
    }

    pub fn execution_context(&self) -> &Option<ContextField> {
        &self.execution_context
    }
}

fn default_context_field() -> Option<ContextField> {
    Some(ContextField::All)
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
    All,
}

/*
 * The field may be: missing, "all", or a context object. 
 * Missing is handled by the Optional<ContextField> from serde. 
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
    call_context: Option<Vec<String>>,
    uid: Option<String>,
    gid: Option<String>,
}

impl Context {
    pub fn call_context(&self) -> &Option<Vec<String>> {
        &self.call_context
    }

    pub fn uid(&self) -> &Option<String> {
        &self.uid
    }

    pub fn gid(&self) -> &Option<String> {
        &self.gid
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
    object_context: Option<ContextField>,
}

impl Object {
    pub fn objects(&self) -> &Vec<String> {
        &self.objects
    }

    pub fn object_context(&self) -> &Option<ContextField> {
        &self.object_context
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;
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
            object_context: Some(ContextField::All),
        }]));
    }

    #[test]
    fn test_deserialize_rw_priv_field_empty_list() {
        let yaml = "[]";
        let result: RWPrivField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, RWPrivField::List(vec![]));
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
            call_context: Some(vec!["domain1".to_string(), "domain2".to_string()]),
            uid: Some("root".to_string()),
            gid: Some("group1".to_string()),
        }));
    }

    #[test]
    fn test_deserialize_context_field_missing_fields() {
        let yaml = "call_context:\n  - domain1\n  - domain2";
        let result: ContextField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(result, ContextField::Context(Context {
            call_context: Some(vec!["domain1".to_string(), "domain2".to_string()]),
            uid: None,
            gid: None,
        }));
    }

    #[test]
    fn test_deserialize_invalid_data() {
        let yaml = "invalid_data";
        let result: Result<CallRetPrivField, _> = serde_yaml::from_str(yaml);
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
      subject:
        name: subject1
        subjects: [subject1, subject2]
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
                    subject: SubjectDomain {
                        name: "subject1".to_string(),
                        subjects: vec!["subject1".to_string(), "subject2".to_string()],
                    },
                    execution_context: Some(ContextField::All),
                },
                can_call: Some(CallRetPrivField::All),
                can_return: Some(CallRetPrivField::All),
                can_read: Some(RWPrivField::All),
                can_write: Some(RWPrivField::All),
            }],
        });
    }
}