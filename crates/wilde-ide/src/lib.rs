//! Native IDE state pilot; execution and file authority stay with the host.
pub mod editor_document;
pub mod terminal_model;
pub use editor_document::{EditorDocument, EditorError, EditorLimits, SaveCandidate};
pub use terminal_model::{TerminalError, TerminalModel};
