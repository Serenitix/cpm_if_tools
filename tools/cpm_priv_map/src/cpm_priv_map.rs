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
    can_call: Option<Vec<String>>,
    can_return: Option<Vec<String>>,
    can_read: Option<Vec<AccessDescriptor>>,
    can_write: Option<Vec<AccessDescriptor>>,
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
    #[serde(default)]
    execution_context: OptionalContextField,
}

/*
 * The context field is used for execution_context and subject_context fields 
 * of the Principal and Object objects. The logic of the grammar allows for an 
 * optional context field that is either non-existent and defaults to "all" or 
 * as a context object. This enum allows for the either defined or all, which 
 * then leads to custom serialization and deserialization.
 */
#[derive(Debug)]
enum OptionalContextField {
    Context(Context),
    All,
}

/*
 * Implement the default value for the OptionalContextField enum.
 */
impl Default for OptionalContextField {
    fn default() -> Self {
        OptionalContextField::All
    }
}

impl<'de> Deserialize<'de> for OptionalContextField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<Context> = Option::deserialize(deserializer)?;
        Ok(opt.map_or(OptionalContextField::All, OptionalContextField::Context))
    }
}

impl Serialize for OptionalContextField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OptionalContextField::Context(ctx) => ctx.serialize(serializer),
            OptionalContextField::All => serializer.serialize_none(),
        }
    }
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