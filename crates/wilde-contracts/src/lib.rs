//! Native pilot protocol; legacy endpoints and credentials are never reused.
use serde::{Deserialize, Deserializer, Serialize};

pub const PROTOCOL_MAJOR: u16 = 1;
pub const MAX_FRAME_BYTES: usize = 65536;
pub const MAX_WIRE_BYTES: usize = MAX_FRAME_BYTES * 5 + 4096;
pub const MAX_ID_BYTES: usize = 240;
pub const DEV_APPLICATION_ID: &str = "systems.wilde.native-rust.dev";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessVerdict {
    Live,
    Unverifiable,
    Exited,
}
impl<'de> Deserialize<'de> for ProcessVerdict {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "live" => Self::Live,
            "exited" => Self::Exited,
            _ => Self::Unverifiable,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionIdentity {
    pub session_id: String,
    pub execution_host_id: String,
    pub workspace_id: String,
    pub generation: u64,
}
impl SessionIdentity {
    pub fn validate(&self) -> Result<(), ContractError> {
        for value in [
            &self.session_id,
            &self.execution_host_id,
            &self.workspace_id,
        ] {
            validate_id(value)?;
        }
        Ok(())
    }
}
fn validate_id(value: &str) -> Result<(), ContractError> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES {
        return Err(ContractError::InvalidIdentity);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalFrame {
    pub identity: SessionIdentity,
    pub sequence: u64,
    pub payload: Vec<u8>,
}
impl TerminalFrame {
    pub fn validate(&self) -> Result<(), ContractError> {
        self.identity.validate()?;
        if self.payload.len() > MAX_FRAME_BYTES {
            return Err(ContractError::FrameTooLarge);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub protocol_major: u16,
    pub request_id: String,
    pub body: T,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    UnsupportedVersion,
    InvalidIdentity,
    SnapshotRequired,
    FrameTooLarge,
    WireTooLarge,
    InvalidMessage,
    HostMismatch,
}
impl<T> Envelope<T> {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.protocol_major != PROTOCOL_MAJOR {
            return Err(ContractError::UnsupportedVersion);
        }
        validate_id(&self.request_id)
    }
}

// The caller supplies a host already authorized by its transport; payload IDs grant no authority.
pub fn decode_terminal_envelope_for_host(
    bytes: &[u8],
    authorized_host: &str,
) -> Result<Envelope<TerminalFrame>, ContractError> {
    if bytes.len() > MAX_WIRE_BYTES {
        return Err(ContractError::WireTooLarge);
    }
    validate_id(authorized_host)?;
    let envelope: Envelope<TerminalFrame> =
        serde_json::from_slice(bytes).map_err(|_| ContractError::InvalidMessage)?;
    envelope.validate()?;
    envelope.body.validate()?;
    if envelope.body.identity.execution_host_id != authorized_host {
        return Err(ContractError::HostMismatch);
    }
    Ok(envelope)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamCursor {
    identity: SessionIdentity,
    next_sequence: u64,
}
impl StreamCursor {
    pub fn from_snapshot(
        identity: SessionIdentity,
        next_sequence: u64,
    ) -> Result<Self, ContractError> {
        identity.validate()?;
        Ok(Self {
            identity,
            next_sequence,
        })
    }
    pub fn accept(&mut self, frame: &TerminalFrame) -> Result<(), ContractError> {
        frame.validate()?;
        if frame.identity != self.identity || frame.sequence != self.next_sequence {
            return Err(ContractError::SnapshotRequired);
        }
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(ContractError::SnapshotRequired)?;
        Ok(())
    }
    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
}
pub fn verdict_for_observer(host_answer: Option<ProcessVerdict>) -> ProcessVerdict {
    host_answer.unwrap_or(ProcessVerdict::Unverifiable)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> SessionIdentity {
        SessionIdentity {
            session_id: "demo-session".into(),
            execution_host_id: "local-dev".into(),
            workspace_id: "demo-folder".into(),
            generation: 1,
        }
    }
    fn envelope() -> Envelope<TerminalFrame> {
        Envelope {
            protocol_major: PROTOCOL_MAJOR,
            request_id: "request-demo".into(),
            body: TerminalFrame {
                identity: identity(),
                sequence: 0,
                payload: b"\x1b[31mhello\r\n".to_vec(),
            },
        }
    }
    #[test]
    fn lost_host_contact_cannot_invent_process_exit() {
        assert_eq!(
            verdict_for_observer(Some(ProcessVerdict::Live)),
            ProcessVerdict::Live
        );
        assert_eq!(verdict_for_observer(None), ProcessVerdict::Unverifiable);
        assert_eq!(
            verdict_for_observer(Some(ProcessVerdict::Exited)),
            ProcessVerdict::Exited
        );
    }
    #[test]
    fn unknown_verdict_preserves_reply_but_wrong_type_is_rejected() {
        assert_eq!(
            serde_json::from_str::<ProcessVerdict>("\"future\"").unwrap(),
            ProcessVerdict::Unverifiable
        );
        assert!(serde_json::from_str::<ProcessVerdict>("42").is_err());
        assert!(serde_json::from_str::<ProcessVerdict>("null").is_err());
    }
    #[test]
    fn gap_does_not_advance_cursor_and_replay_requires_snapshot() {
        let id = identity();
        let mut cursor = StreamCursor::from_snapshot(id.clone(), 5).unwrap();
        let mut frame = TerminalFrame {
            identity: id,
            sequence: 6,
            payload: b"\x1b[".to_vec(),
        };
        assert_eq!(cursor.accept(&frame), Err(ContractError::SnapshotRequired));
        assert_eq!(cursor.next_sequence(), 5);
        frame.sequence = 5;
        cursor.accept(&frame).unwrap();
        assert_eq!(cursor.accept(&frame), Err(ContractError::SnapshotRequired));
    }
    #[test]
    fn restarted_session_and_oversized_payload_do_not_consume_cursor() {
        let id = identity();
        let mut cursor = StreamCursor::from_snapshot(id.clone(), 0).unwrap();
        let mut frame = TerminalFrame {
            identity: id,
            sequence: 0,
            payload: vec![],
        };
        frame.identity.generation = 2;
        assert_eq!(cursor.accept(&frame), Err(ContractError::SnapshotRequired));
        frame.identity.generation = 1;
        frame.payload = vec![0; MAX_FRAME_BYTES + 1];
        assert_eq!(cursor.accept(&frame), Err(ContractError::FrameTooLarge));
        assert_eq!(cursor.next_sequence(), 0);
    }
    #[test]
    fn wire_ingress_preserves_bytes_and_unknown_optional_fields() {
        let env = envelope();
        let mut value = serde_json::to_value(&env).unwrap();
        value["future_optional"] = serde_json::json!(true);
        let bytes = serde_json::to_vec(&value).unwrap();
        assert_eq!(
            decode_terminal_envelope_for_host(&bytes, "local-dev").unwrap(),
            env
        );
        assert_eq!(
            decode_terminal_envelope_for_host(&bytes, "other-host"),
            Err(ContractError::HostMismatch)
        );
    }
    #[test]
    fn bounded_ingress_rejects_invalid_identity_before_dispatch() {
        let mut env = envelope();
        env.body.identity.session_id = " ".into();
        assert_eq!(
            decode_terminal_envelope_for_host(&serde_json::to_vec(&env).unwrap(), "local-dev"),
            Err(ContractError::InvalidIdentity)
        );
        env.body.identity = identity();
        env.body.identity.workspace_id = "x".repeat(MAX_ID_BYTES + 1);
        assert_eq!(env.body.validate(), Err(ContractError::InvalidIdentity));
        assert_eq!(
            decode_terminal_envelope_for_host(&vec![b'x'; MAX_WIRE_BYTES + 1], "local-dev"),
            Err(ContractError::WireTooLarge)
        );
    }
    #[test]
    fn unsupported_major_fails_before_execution() {
        let env = Envelope {
            protocol_major: 2,
            request_id: "demo".into(),
            body: (),
        };
        assert_eq!(env.validate(), Err(ContractError::UnsupportedVersion));
    }
}
