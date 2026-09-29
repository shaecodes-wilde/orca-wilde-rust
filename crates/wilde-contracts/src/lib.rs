//! Versioned pilot contracts; no connection to the legacy runtime.
use serde::{Deserialize, Serialize};

pub const PROTOCOL_MAJOR: u16 = 1;
pub const MAX_FRAME_BYTES: usize = 65536;
pub const DEV_APPLICATION_ID: &str = "systems.wilde.native-rust.dev";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessVerdict { Live, Unverifiable, Exited }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionIdentity {
    pub session_id: String,
    pub execution_host_id: String,
    pub workspace_id: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalFrame {
    pub identity: SessionIdentity,
    pub sequence: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub protocol_major: u16,
    pub request_id: String,
    pub body: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError { UnsupportedVersion, InvalidIdentity, SnapshotRequired, FrameTooLarge }

impl<T> Envelope<T> {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.protocol_major != PROTOCOL_MAJOR { return Err(ContractError::UnsupportedVersion); }
        if self.request_id.is_empty() || self.request_id.len() > 240 { return Err(ContractError::InvalidIdentity); }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamCursor { identity: SessionIdentity, next_sequence: u64 }
impl StreamCursor {
    pub fn from_snapshot(identity: SessionIdentity, next_sequence: u64) -> Result<Self, ContractError> {
        if identity.session_id.is_empty() || identity.execution_host_id.is_empty() || identity.workspace_id.is_empty() {
            return Err(ContractError::InvalidIdentity);
        }
        Ok(Self { identity, next_sequence })
    }
    pub fn accept(&mut self, frame: &TerminalFrame) -> Result<(), ContractError> {
        if frame.payload.len() > MAX_FRAME_BYTES { return Err(ContractError::FrameTooLarge); }
        if frame.identity != self.identity || frame.sequence != self.next_sequence { return Err(ContractError::SnapshotRequired); }
        self.next_sequence = self.next_sequence.checked_add(1).ok_or(ContractError::SnapshotRequired)?;
        Ok(())
    }
    pub fn next_sequence(&self) -> u64 { self.next_sequence }
}

pub fn verdict_for_observer(host_answer: Option<ProcessVerdict>) -> ProcessVerdict {
    host_answer.unwrap_or(ProcessVerdict::Unverifiable)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> SessionIdentity { SessionIdentity { session_id: "demo-session".into(), execution_host_id: "local-dev".into(), workspace_id: "demo-folder".into(), generation: 1 } }
    #[test]
    fn lost_host_contact_cannot_invent_process_exit() {
        assert_eq!(verdict_for_observer(Some(ProcessVerdict::Live)), ProcessVerdict::Live);
        assert_eq!(verdict_for_observer(None), ProcessVerdict::Unverifiable);
        assert_eq!(verdict_for_observer(Some(ProcessVerdict::Exited)), ProcessVerdict::Exited);
    }
    #[test]
    fn gap_does_not_advance_cursor_and_replay_requires_snapshot() {
        let id=identity(); let mut cursor=StreamCursor::from_snapshot(id.clone(), 5).unwrap();
        let mut frame=TerminalFrame { identity: id, sequence: 6, payload: b"\x1b[".to_vec() };
        assert_eq!(cursor.accept(&frame), Err(ContractError::SnapshotRequired));
        assert_eq!(cursor.next_sequence(),5);
        frame.sequence=5; cursor.accept(&frame).unwrap();
        assert_eq!(cursor.accept(&frame),Err(ContractError::SnapshotRequired));
    }
    #[test]
    fn restarted_session_and_oversized_payload_are_rejected_without_consuming() {
        let id=identity(); let mut cursor=StreamCursor::from_snapshot(id.clone(),0).unwrap();
        let mut frame=TerminalFrame { identity:id,sequence:0,payload:vec![] };
        frame.identity.generation=2;
        assert_eq!(cursor.accept(&frame),Err(ContractError::SnapshotRequired));
        frame.identity.generation=1; frame.payload=vec![0; MAX_FRAME_BYTES+1];
        assert_eq!(cursor.accept(&frame),Err(ContractError::FrameTooLarge));
        assert_eq!(cursor.next_sequence(),0);
    }
    #[test]
    fn wire_roundtrip_preserves_escape_bytes_and_unknown_optional_fields() {
        let frame=TerminalFrame { identity:identity(),sequence:0,payload:b"\x1b[31mhello\r\n".to_vec() };
        let env=Envelope { protocol_major:PROTOCOL_MAJOR,request_id:"request-demo".into(),body:frame };
        let mut value=serde_json::to_value(&env).unwrap(); value["future_optional"]=serde_json::json!(true);
        let decoded:Envelope<TerminalFrame>=serde_json::from_value(value).unwrap();
        decoded.validate().unwrap(); assert_eq!(decoded,env);
    }
    #[test]
    fn unsupported_major_fails_before_execution() {
        let env=Envelope { protocol_major:2,request_id:"demo".into(),body:() };
        assert_eq!(env.validate(),Err(ContractError::UnsupportedVersion));
    }
}
