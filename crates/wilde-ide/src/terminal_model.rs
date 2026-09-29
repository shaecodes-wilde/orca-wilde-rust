use wilde_contracts::{ContractError, SessionIdentity, StreamCursor, TerminalFrame};

pub const MAX_ROWS: u16 = 200;
pub const MAX_COLUMNS: u16 = 400;
pub const MAX_SCROLLBACK_ROWS: usize = 10_000;
pub const MAX_RETAINED_CELLS: usize = 2_000_000;
/// Pilot lifetime admission limit, not a production terminal retention policy.
pub const MAX_SESSION_INPUT_BYTES: usize = 4 * 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalError {
    InvalidDimensions,
    RetentionLimit,
    LifetimeByteLimit,
    Contract(ContractError),
}

pub struct TerminalModel {
    parser: vt100::Parser,
    cursor: StreamCursor,
    scrollback_rows: usize,
    widest_columns: u16,
    snapshot_required: bool,
    admitted_bytes: usize,
    resource_exhausted: bool,
}

impl TerminalModel {
    /// Only for a newly created host session, before any output has been observed.
    pub fn new_session(
        identity: SessionIdentity,
        rows: u16,
        cols: u16,
        scrollback_rows: usize,
    ) -> Result<Self, TerminalError> {
        validate_size(rows, cols, scrollback_rows)?;
        if identity.generation == 0 {
            return Err(TerminalError::Contract(ContractError::InvalidIdentity));
        }
        let cursor = StreamCursor::from_snapshot(identity, 0).map_err(TerminalError::Contract)?;
        Ok(Self {
            parser: vt100::Parser::new(rows, cols, scrollback_rows),
            cursor,
            scrollback_rows,
            widest_columns: cols,
            snapshot_required: false,
            admitted_bytes: 0,
            resource_exhausted: false,
        })
    }

    pub fn accept_frame(&mut self, frame: &TerminalFrame) -> Result<(), TerminalError> {
        if self.resource_exhausted {
            return Err(TerminalError::LifetimeByteLimit);
        }
        if self.snapshot_required {
            return Err(TerminalError::Contract(ContractError::SnapshotRequired));
        }
        let mut accepted_cursor = self.cursor.clone();
        if let Err(error) = accepted_cursor.accept(frame) {
            if error == ContractError::SnapshotRequired {
                self.snapshot_required = true;
            }
            return Err(TerminalError::Contract(error));
        }
        if frame.payload.len() > MAX_SESSION_INPUT_BYTES - self.admitted_bytes {
            self.resource_exhausted = true;
            return Err(TerminalError::LifetimeByteLimit);
        }
        self.cursor = accepted_cursor;
        self.admitted_bytes += frame.payload.len();
        self.parser.process(&frame.payload);
        Ok(())
    }

    pub fn screen(&self) -> &vt100::Screen {
        self.parser.screen()
    }

    pub fn contents(&self) -> String {
        self.screen().contents()
    }

    pub fn next_sequence(&self) -> u64 {
        self.cursor.next_sequence()
    }

    pub fn snapshot_required(&self) -> bool {
        self.snapshot_required
    }

    pub fn admitted_bytes(&self) -> usize {
        self.admitted_bytes
    }

    pub fn resource_exhausted(&self) -> bool {
        self.resource_exhausted
    }

    /// Caller must order this with the corresponding host PTY resize.
    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<(), TerminalError> {
        validate_size(rows, cols, self.scrollback_rows)?;
        let widest_columns = self.widest_columns.max(cols);
        validate_size(rows, widest_columns, self.scrollback_rows)?;
        self.parser.screen_mut().set_size(rows, cols);
        self.widest_columns = widest_columns;
        Ok(())
    }

    pub fn set_scrollback(&mut self, rows: usize) {
        self.parser
            .screen_mut()
            .set_scrollback(rows.min(self.scrollback_rows));
    }
}

fn validate_size(rows: u16, cols: u16, scrollback: usize) -> Result<(), TerminalError> {
    if rows == 0 || cols == 0 || rows > MAX_ROWS || cols > MAX_COLUMNS {
        return Err(TerminalError::InvalidDimensions);
    }
    if scrollback > MAX_SCROLLBACK_ROWS
        || (2 * usize::from(rows) + scrollback) * usize::from(cols) > MAX_RETAINED_CELLS
    {
        return Err(TerminalError::RetentionLimit);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
