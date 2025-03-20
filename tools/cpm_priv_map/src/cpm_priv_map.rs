use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::object_identifer::ObjectID;
// TODO make sure to specify yaml serialization and desserialization

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

#[derive(Debug, Deserialize, Serialize)]
struct Privilege {
    principal: Principal,
    #[serde(default = "default_all")]
    can_call: Option<Vec<String>>,
    #[serde(default = "default_all")]
    can_return: Option<Vec<String>>,
    #[serde(default = "default_all")]
    can_read: Option<Vec<AccessDescriptor>>,
    #[serde(default = "default_all")]
    can_write: Option<Vec<AccessDescriptor>>,
}

fn default_all() -> Option<Vec<String>> {
    Some(vec!["all".to_string()])
}

/*
 * Principal ::= { subject: SubjectDomain, ? execution context: Context | all }
 */
#[derive(Debug, Deserialize, Serialize)]
struct Principal {
    // TODO make this work correctly: point to a subject domain 
    //   eg: subject: SubjectDomain, // but a reference to a subject domain
    //   well the grammar specifies it like this right now. so i will be 
    //   faithful to the grammar and propose changes after one version
    subject: SubjectDomain,
    #[serde(default = "default_execution_context")]
    execution_context: OptionalContextField,
}

fn default_execution_context() -> OptionalContextField {
    OptionalContextField::All
}

/*
 * The context field is used for execution_context and subject_context fields 
 * of the Principal and Object objects. The logic of the grammar allows for an 
 * optional context field that is either non-existent and defaults to "all" or 
 * as a context object. This enum allows for either a defined context or "all", 
 * which then leads to simpler serialization and deserialization.
 */
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum OptionalContextField {
    Context(Context),
    All(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    call_context: Option<Vec<String>>,
    uid: Option<String>,
    gid: Option<String>,
}

/*
 * The AccessDescriptor struct represents access permissions for objects.
 * 
 * Grammar:
 * 
 * AccessDescriptor:
 *   objects: [ObjectIdentifier]
 *   object_context: ?Context
 * 
 * If the object_context field is omitted, it defaults to "all".
 */
#[derive(Debug, Deserialize, Serialize)]
struct AccessDescriptor {
    objects: Vec<ObjectIdentifier>,
    #[serde(default)]
    object_context: OptionalContextField,
}