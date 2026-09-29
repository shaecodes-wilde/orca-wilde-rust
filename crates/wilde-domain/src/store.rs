use crate::model::bounded;
use crate::{
    Error, Export, MAX_JSON_BYTES, MAX_MUTATIONS, MAX_RECORDS, Record, Request, Result, Snapshot,
};
use rusqlite::{
    Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior, params,
};
use std::{collections::HashSet, path::Path, time::Duration};

const APPLICATION_ID: i64 = 1464099891;

/// One blocking write authority. Call on a dedicated service thread, never the UI thread.
pub struct Store {
    connection: Connection,
}
impl Store {
    pub fn in_memory(runtime_owner_id: &str) -> Result<Self> {
        Self::initialize(Connection::open_in_memory()?, runtime_owner_id, false)
    }
    /// Requires an explicit native development filename; no profile discovery or fallback.
    pub fn open_dev(path: &Path, runtime_owner_id: &str) -> Result<Self> {
        let marked = path
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|name| name.ends_with(".wilde-dev.sqlite"));
        if !path.is_absolute() || !marked {
            return Err(Error::WrongDatabase);
        }
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        Self::initialize(connection, runtime_owner_id, true)
    }
    fn initialize(mut connection: Connection, owner: &str, persistent: bool) -> Result<Self> {
        bounded(owner, 240, true)?;
        let app_id: i64 = connection.query_row("PRAGMA application_id", [], |r| r.get(0))?;
        let version: i64 = connection.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        let tables: i64 = connection.query_row(
            "SELECT count(*) FROM sqlite_master WHERE name NOT LIKE 'sqlite_%'",
            [],
            |r| r.get(0),
        )?;
        if app_id == 0 && version == 0 && tables == 0 {
            let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
            tx.execute_batch(include_str!("schema.sql"))?;
            tx.execute("INSERT INTO metadata VALUES (1, ?1, 0)", [owner])?;
            tx.commit()?;
        } else if app_id != APPLICATION_ID || version != 1 {
            return Err(Error::WrongDatabase);
        }
        let stored_owner: String = connection.query_row(
            "SELECT runtime_owner_id FROM metadata WHERE singleton=1",
            [],
            |r| r.get(0),
        )?;
        if stored_owner != owner {
            return Err(Error::WrongDatabase);
        }
        connection.busy_timeout(Duration::from_secs(1))?;
        connection.execute_batch("PRAGMA foreign_keys=ON")?;
        if persistent {
            connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL")?;
        }
        validate_relations(&connection)?;
        Ok(Self { connection })
    }
    pub fn snapshot(&mut self) -> Result<Snapshot> {
        let tx = self.connection.transaction()?;
        let snapshot = load_snapshot(&tx)?;
        tx.commit()?;
        Ok(snapshot)
    }
    /// A native pilot export, intentionally distinct from the legacy backup format.
    pub fn export_json(&mut self) -> Result<String> {
        let snapshot = self.snapshot()?;
        let value = serde_json::to_string(&Export {
            format: "wilde-native-synthetic".into(),
            schema_version: 1,
            records: snapshot.records,
        })?;
        if value.len() > MAX_JSON_BYTES {
            return Err(Error::Capacity);
        }
        Ok(value)
    }
    pub fn apply(&mut self, request: Request) -> Result<Snapshot> {
        validate_request(&request.request_id, request.expected_revision)?;
        if request.records.is_empty() || request.records.len() > MAX_MUTATIONS {
            return Err(Error::Capacity);
        }
        validate_records(&request.records, false)?;
        let payload =
            serde_json::to_string(&("save", request.expected_revision, &request.records))?;
        self.transact(
            &request.request_id,
            request.expected_revision,
            &payload,
            |tx| {
                for candidate in &request.records {
                    let prior = read_record(tx, candidate.id().as_str())?;
                    if prior.as_ref().map_or(0, Record::revision) != candidate.revision() {
                        return Err(Error::RevisionConflict);
                    }
                    if let Some(prior) = &prior {
                        if prior.kind() != candidate.kind() {
                            return Err(Error::Invalid("Record type cannot change".into()));
                        }
                        if matches!(candidate, Record::Assignment { .. })
                            || prior.client() != candidate.client()
                        {
                            return Err(Error::NeedsPreview);
                        }
                    }
                    let mut saved = candidate.clone();
                    saved.advance()?;
                    write_record(tx, &saved)?;
                }
                Ok(())
            },
        )
    }
    /// Synthetic merge only: identical records skip, conflicting IDs fail atomically.
    pub fn import_json(
        &mut self,
        request_id: &str,
        expected_revision: u64,
        content: &str,
    ) -> Result<Snapshot> {
        validate_request(request_id, expected_revision)?;
        if content.len() > MAX_JSON_BYTES {
            return Err(Error::Capacity);
        }
        let export: Export = serde_json::from_str(content)?;
        if export.format != "wilde-native-synthetic" || export.schema_version != 1 {
            return Err(Error::UnsupportedSchema);
        }
        validate_records(&export.records, true)?;
        let payload = serde_json::to_string(&("import", expected_revision, &export))?;
        self.transact(request_id, expected_revision, &payload, |tx| {
            for record in &export.records {
                if let Some(prior) = read_record(tx, record.id().as_str())? {
                    if prior != *record {
                        return Err(Error::Invalid(
                            "Import conflicts with existing record".into(),
                        ));
                    }
                } else {
                    write_record(tx, record)?;
                }
            }
            Ok(())
        })
    }
    fn transact(
        &mut self,
        id: &str,
        revision: u64,
        payload: &str,
        work: impl FnOnce(&Transaction<'_>) -> Result<()>,
    ) -> Result<Snapshot> {
        if payload.len() > MAX_JSON_BYTES {
            return Err(Error::Capacity);
        }
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let prior: Option<(String, String)> = tx
            .query_row(
                "SELECT payload,result FROM requests WHERE id=?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        if let Some((original, result)) = prior {
            if original != payload {
                return Err(Error::RequestIdConflict);
            }
            return Ok(serde_json::from_str(&result)?);
        }
        let current: u64 =
            tx.query_row("SELECT revision FROM metadata WHERE singleton=1", [], |r| {
                r.get(0)
            })?;
        if current != revision {
            return Err(Error::RevisionConflict);
        }
        let requests: usize = tx.query_row("SELECT count(*) FROM requests", [], |r| r.get(0))?;
        if requests >= MAX_RECORDS || current >= i64::MAX as u64 {
            return Err(Error::Capacity);
        }
        work(&tx)?;
        validate_relations(&tx)?;
        tx.execute(
            "UPDATE metadata SET revision=revision+1 WHERE singleton=1",
            [],
        )?;
        let result = load_snapshot(&tx)?;
        let saved = serde_json::to_string(&result)?;
        if saved.len() > MAX_JSON_BYTES {
            return Err(Error::Capacity);
        }
        let log_bytes: usize = tx.query_row("SELECT coalesce(sum(length(CAST(payload AS BLOB))+length(CAST(result AS BLOB))),0) FROM requests", [], |r| r.get(0))?;
        if log_bytes + payload.len() + saved.len() > 64 * 1024 * 1024 {
            return Err(Error::Capacity);
        }
        tx.execute(
            "INSERT INTO requests VALUES (?1,?2,?3)",
            params![id, payload, saved],
        )?;
        tx.commit()?;
        Ok(result)
    }
}
fn validate_request(id: &str, revision: u64) -> Result<()> {
    bounded(id, 240, true)?;
    if revision > i64::MAX as u64 {
        return Err(Error::Invalid("Invalid store revision".into()));
    }
    Ok(())
}
fn validate_records(records: &[Record], importing: bool) -> Result<()> {
    if records.len() > MAX_RECORDS {
        return Err(Error::Capacity);
    }
    let mut ids = HashSet::new();
    for record in records {
        record.validate(importing)?;
        if !ids.insert(record.id()) {
            return Err(Error::Invalid("Duplicate record ID".into()));
        }
    }
    Ok(())
}
fn read_record(connection: &Connection, id: &str) -> Result<Option<Record>> {
    let body: Option<String> = connection
        .query_row("SELECT body FROM records WHERE id=?1", [id], |r| r.get(0))
        .optional()?;
    body.map(|body| serde_json::from_str(&body).map_err(Error::from))
        .transpose()
}
fn write_record(connection: &Connection, record: &Record) -> Result<()> {
    let key = if let Record::Assignment { target, .. } = record {
        Some(serde_json::to_string(&(
            &target.runtime_owner_id,
            &target.host_id,
            &target.kind,
            &target.stable_id,
        ))?)
    } else {
        None
    };
    connection.execute("INSERT INTO records(id,kind,client_id,project_id,revision,body,target_key) VALUES (?1,?2,?3,?4,?5,?6,?7) ON CONFLICT(id) DO UPDATE SET body=excluded.body,revision=excluded.revision,client_id=excluded.client_id,project_id=excluded.project_id,target_key=excluded.target_key",
        params![record.id().as_str(), record.kind(), record.client().map(|id| id.as_str()), record.project().map(|id| id.as_str()), record.revision(), serde_json::to_string(record)?, key])?;
    Ok(())
}
fn validate_relations(connection: &Connection) -> Result<()> {
    let invalid: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM records r LEFT JOIN records c ON c.id=r.client_id LEFT JOIN records p ON p.id=r.project_id WHERE (r.client_id IS NOT NULL AND (c.kind IS NULL OR c.kind!='client')) OR (r.project_id IS NOT NULL AND (p.kind IS NULL OR p.kind!='project' OR p.client_id!=r.client_id)))", [], |r| r.get(0))?;
    if invalid {
        return Err(Error::Invalid(
            "Missing or cross-client relationship".into(),
        ));
    }
    Ok(())
}
fn load_snapshot(connection: &Connection) -> Result<Snapshot> {
    let (runtime_owner_id, revision) = connection.query_row(
        "SELECT runtime_owner_id,revision FROM metadata WHERE singleton=1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    let mut statement = connection.prepare("SELECT body FROM records ORDER BY id LIMIT 10001")?;
    let bodies = statement.query_map([], |r| r.get::<_, String>(0))?;
    let mut records = Vec::new();
    for body in bodies {
        records.push(serde_json::from_str(&body?)?);
    }
    if records.len() > MAX_RECORDS {
        return Err(Error::Capacity);
    }
    Ok(Snapshot {
        runtime_owner_id,
        revision,
        records,
    })
}
