use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use serde::ser::{self, Serializer};
use std::fmt;

#[derive(Debug)]
pub struct ObjectIdentifier {
    pub alloc_type: AllocType,
    pub path: String,
    pub lineno: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AllocType {
    Global,
    Local,
    Heap,
    StackFrame,
    StackRegion,
    IO,
    Other
}

impl Serialize for ObjectIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!(
            "{}:{}:{}:{}",
            self.id, self.loc_concrete, self.loc_symbolic, self.alloc_type
        );
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ObjectIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err(de::Error::custom("invalid object identifier format"));
        }
        Ok(ObjectIdentifier {
            id: parts[0].to_string(),
            loc_concrete: parts[1].to_string(),
            loc_symbolic: parts[2].to_string(),
            alloc_type: parts[3].parse().map_err(de::Error::custom)?,
        })
    }
}

impl fmt::Display for AllocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocType::Global => write!(f, "Global"),
            AllocType::Local => write!(f, "Local"),
            AllocType::Heap => write!(f, "Heap"),
            AllocType::Stack => write!(f, "Stack"),
        }
    }
}

impl std::str::FromStr for AllocType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Global" => Ok(AllocType::Global),
            "Local" => Ok(AllocType::Local),
            "Heap" => Ok(AllocType::Heap),
            "Stack" => Ok(AllocType::Stack),
            _ => Err(format!("invalid alloc type: {}", s)),
        }
    }
}