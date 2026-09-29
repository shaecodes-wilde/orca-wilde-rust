use crate::{Error, Result};
use serde::{Deserialize, Serialize};

pub const MAX_RECORDS: usize = 10_000;
pub const MAX_MUTATIONS: usize = 1_000;
pub const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct RecordId(String);
impl RecordId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        let valid = value.len() == 36
            && value.bytes().enumerate().all(|(i, b)| {
                if [8, 13, 18, 23].contains(&i) {
                    b == b'-'
                } else {
                    b.is_ascii_hexdigit()
                }
            });
        if !valid {
            return Err(Error::Invalid("Record ID must be a UUID".into()));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for RecordId {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetKind {
    Folder,
    Worktree,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceTarget {
    pub kind: TargetKind,
    pub runtime_owner_id: String,
    pub host_id: String,
    pub stable_id: String,
    pub locator: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Record {
    Client {
        id: RecordId,
        revision: u64,
        name: String,
        owner: String,
    },
    Project {
        id: RecordId,
        revision: u64,
        client_id: RecordId,
        title: String,
    },
    Assignment {
        id: RecordId,
        revision: u64,
        client_id: RecordId,
        project_id: Option<RecordId>,
        target: WorkspaceTarget,
        reason: String,
    },
}
impl Record {
    pub fn id(&self) -> &RecordId {
        match self {
            Self::Client { id, .. } | Self::Project { id, .. } | Self::Assignment { id, .. } => id,
        }
    }
    pub fn revision(&self) -> u64 {
        match self {
            Self::Client { revision, .. }
            | Self::Project { revision, .. }
            | Self::Assignment { revision, .. } => *revision,
        }
    }
    pub(crate) fn advance(&mut self) -> Result<()> {
        let revision = match self {
            Self::Client { revision, .. }
            | Self::Project { revision, .. }
            | Self::Assignment { revision, .. } => revision,
        };
        *revision = revision
            .checked_add(1)
            .filter(|r| *r <= i64::MAX as u64)
            .ok_or(Error::Capacity)?;
        Ok(())
    }
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::Client { .. } => "client",
            Self::Project { .. } => "project",
            Self::Assignment { .. } => "assignment",
        }
    }
    pub(crate) fn client(&self) -> Option<&RecordId> {
        match self {
            Self::Client { .. } => None,
            Self::Project { client_id, .. } | Self::Assignment { client_id, .. } => Some(client_id),
        }
    }
    pub(crate) fn project(&self) -> Option<&RecordId> {
        match self {
            Self::Assignment { project_id, .. } => project_id.as_ref(),
            _ => None,
        }
    }
    pub(crate) fn validate(&self, importing: bool) -> Result<()> {
        if self.revision() > i64::MAX as u64 || (importing && self.revision() == 0) {
            return Err(Error::Invalid("Invalid record revision".into()));
        }
        match self {
            Self::Client { name, owner, .. } => {
                bounded(name, 240, true)?;
                bounded(owner, 240, false)?;
            }
            Self::Project { title, .. } => bounded(title, 240, true)?,
            Self::Assignment { target, reason, .. } => {
                bounded(&target.runtime_owner_id, 240, true)?;
                bounded(&target.host_id, 240, true)?;
                bounded(&target.stable_id, 1024, true)?;
                bounded(&target.locator, 2048, false)?;
                bounded(&target.name, 240, true)?;
                bounded(reason, 240, false)?;
            }
        }
        Ok(())
    }
}
pub(crate) fn bounded(value: &str, max: usize, required: bool) -> Result<()> {
    if value.len() > max || (required && value.trim().is_empty()) || value.contains('\0') {
        return Err(Error::Invalid("Missing or oversized field".into()));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub request_id: String,
    pub expected_revision: u64,
    pub records: Vec<Record>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub runtime_owner_id: String,
    pub revision: u64,
    pub records: Vec<Record>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Export {
    pub format: String,
    pub schema_version: u32,
    pub records: Vec<Record>,
}
