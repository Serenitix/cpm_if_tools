use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::fmt;
use crate::object_identifer::ObjectID;
// TODO make sure to specify yaml serialization and deserialization

#[derive(Debug, Deserialize, Serialize)]
pub struct CPMPrivMap {
    object_map: Vec<ObjectDomain>,
    subject_map: Vec<SubjectDomain>,
    privileges: Vec<Privilege>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ObjectDomain {
    name: String,
    objects: Vec<String>,
    //objects: Vec<ObjectID>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubjectDomain {
    name: String,
    subjects: Vec<String>,
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
#[derive(Debug, Deserialize, Serialize)]
struct Privilege {
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

fn default_callret_priv_field() -> Option<CallRetPrivField> {
    Some(CallRetPrivField::All)
}

#[derive(Debug, Serialize)]
enum CallRetPrivField {
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

#[derive(Debug, Serialize)]
enum RWPrivField {
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
#[derive(Debug, Deserialize, Serialize)]
struct Principal {
    // TODO make this work correctly: point to a subject domain 
    //   eg: subject: SubjectDomain, // but a reference to a subject domain
    //   well the grammar specifies it like this right now. so i will be 
    //   faithful to the grammar and propose changes after one version
    subject: SubjectDomain,
    #[serde(default = "default_context_field")]
    execution_context: Option<ContextField>,
}

fn default_context_field() -> Option<ContextField> {
    Some(ContextField::All)
}

/*
 * The context field is used for execution_context and subject_context fields 
 * of the Principal and Object objects. The logic of the grammar allows for an 
 * optional context field that is either non-existent and defaults to "all" or 
 * as a context object. This enum allows for either a defined context or "all", 
 * which then leads to simpler serialization and deserialization.
 */
#[derive(Debug, Serialize)]
enum ContextField {
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
#[derive(Debug, Deserialize, Serialize)]
struct Context {
    call_context: Option<Vec<String>>,
    uid: Option<String>,
    gid: Option<String>,
}

/*
 * Grammar: Object ::= { objects: [ ObjectDomainName ] | all
 *                     ? object_context: Context | all }
 */
#[derive(Debug, Deserialize, Serialize)]
 struct Object {
    objects: Vec<String>,
    ///objects: Vec<ObjectIdentifier>,
    #[serde(default = "default_context_field")]
    object_context: Option<ContextField>,
}