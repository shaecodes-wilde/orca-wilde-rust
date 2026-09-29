use wilde_domain::{Export, Record, RecordId, TargetKind, WorkspaceTarget};

fn id(number: u32) -> wilde_domain::Result<RecordId> {
    RecordId::new(format!("00000000-0000-4000-8000-{number:012}"))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clients: u32 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "10".into())
        .parse()?;
    if !(1..=2500).contains(&clients) {
        return Err("Use 1..=2500 synthetic clients (four records each)".into());
    }
    let mut records = Vec::new();
    for index in 0..clients {
        let base = index * 4 + 1;
        records.push(Record::Client {
            id: id(base)?,
            revision: 1,
            name: format!("Synthetic client {index}"),
            owner: "Synthetic business owner".into(),
        });
        records.push(Record::Project {
            id: id(base + 1)?,
            revision: 1,
            client_id: id(base)?,
            title: "Synthetic project independent of a repository".into(),
        });
        for (offset, kind) in [(2, TargetKind::Folder), (3, TargetKind::Worktree)] {
            records.push(Record::Assignment {
                id: id(base + offset)?,
                revision: 1,
                client_id: id(base)?,
                project_id: Some(id(base + 1)?),
                target: WorkspaceTarget {
                    kind,
                    runtime_owner_id: "synthetic-runtime".into(),
                    host_id: "ssh:synthetic-host".into(),
                    stable_id: format!("synthetic-target-{}", base + offset),
                    locator: "synthetic-only".into(),
                    name: "Synthetic workspace".into(),
                },
                reason: "Synthetic workload".into(),
            });
        }
    }
    let exported = serde_json::to_string(&Export {
        format: "wilde-native-synthetic".into(),
        schema_version: 1,
        records,
    })?;
    if exported.len() > wilde_domain::MAX_JSON_BYTES {
        return Err("Generated workload exceeds pilot payload limit".into());
    }
    println!("{exported}");
    Ok(())
}
