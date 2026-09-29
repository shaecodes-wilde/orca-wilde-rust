use std::{collections::VecDeque, ops::Range};

pub const MAX_DOCUMENT_BYTES: usize = 1_048_576;
pub const MAX_HISTORY_BYTES: usize = 8_388_608;
pub const MAX_HISTORY_STEPS: usize = 128;

#[derive(Debug, Clone, Copy)]
pub struct EditorLimits {
    pub document_bytes: usize,
    pub history_bytes: usize,
    pub history_steps: usize,
}

impl Default for EditorLimits {
    fn default() -> Self {
        Self {
            document_bytes: MAX_DOCUMENT_BYTES,
            history_bytes: MAX_HISTORY_BYTES,
            history_steps: MAX_HISTORY_STEPS,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorError {
    InvalidLimits,
    DocumentTooLarge,
    InvalidRange,
    Utf8Boundary,
    RevisionExhausted,
    SaveConflict,
}

#[derive(Debug, Clone)]
pub struct SaveCandidate {
    text: String,
    expected_disk: String,
    revision: u64,
}

impl SaveCandidate {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn expected_disk(&self) -> &str {
        &self.expected_disk
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
}

pub struct EditorDocument {
    text: String,
    disk_baseline: String,
    revision: u64,
    undo: VecDeque<String>,
    redo: VecDeque<String>,
    limits: EditorLimits,
}

impl EditorDocument {
    pub fn open(text: String, limits: EditorLimits) -> Result<Self, EditorError> {
        if limits.document_bytes == 0
            || limits.document_bytes > MAX_DOCUMENT_BYTES
            || limits.history_bytes < limits.document_bytes
            || limits.history_bytes > MAX_HISTORY_BYTES
            || limits.history_steps == 0
            || limits.history_steps > MAX_HISTORY_STEPS
        {
            return Err(EditorError::InvalidLimits);
        }
        if text.len() > limits.document_bytes {
            return Err(EditorError::DocumentTooLarge);
        }
        Ok(Self {
            disk_baseline: text.clone(),
            text,
            revision: 0,
            undo: VecDeque::new(),
            redo: VecDeque::new(),
            limits,
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn is_dirty(&self) -> bool {
        self.text != self.disk_baseline
    }
    pub fn retained_history_bytes(&self) -> usize {
        self.undo
            .iter()
            .chain(self.redo.iter())
            .map(String::len)
            .sum()
    }

    pub fn replace(&mut self, range: Range<usize>, replacement: &str) -> Result<(), EditorError> {
        if range.start > range.end || range.end > self.text.len() {
            return Err(EditorError::InvalidRange);
        }
        if !self.text.is_char_boundary(range.start) || !self.text.is_char_boundary(range.end) {
            return Err(EditorError::Utf8Boundary);
        }
        let new_len = self.text.len() - (range.end - range.start);
        if replacement.len() > self.limits.document_bytes.saturating_sub(new_len) {
            return Err(EditorError::DocumentTooLarge);
        }
        if self.text[range.clone()] == *replacement {
            return Ok(());
        }
        let revision = self.next_revision()?;
        self.undo.push_back(self.text.clone());
        self.redo.clear();
        self.text.replace_range(range, replacement);
        self.revision = revision;
        self.trim_history();
        Ok(())
    }

    pub fn undo(&mut self) -> Result<bool, EditorError> {
        if self.undo.is_empty() {
            return Ok(false);
        }
        let revision = self.next_revision()?;
        let previous = self.undo.pop_back().expect("checked nonempty history");
        self.redo
            .push_back(std::mem::replace(&mut self.text, previous));
        self.revision = revision;
        self.trim_history();
        Ok(true)
    }

    pub fn redo(&mut self) -> Result<bool, EditorError> {
        if self.redo.is_empty() {
            return Ok(false);
        }
        let revision = self.next_revision()?;
        let next = self.redo.pop_back().expect("checked nonempty history");
        self.undo.push_back(std::mem::replace(&mut self.text, next));
        self.revision = revision;
        self.trim_history();
        Ok(true)
    }

    /// The execution host must compare again atomically when writing the candidate.
    pub fn prepare_save(&self, observed_disk: &str) -> Result<SaveCandidate, EditorError> {
        if observed_disk != self.disk_baseline {
            return Err(EditorError::SaveConflict);
        }
        Ok(SaveCandidate {
            text: self.text.clone(),
            expected_disk: self.disk_baseline.clone(),
            revision: self.revision,
        })
    }

    /// Call only after the host confirms this exact candidate was durably saved.
    pub fn acknowledge_saved(&mut self, candidate: SaveCandidate) -> Result<(), EditorError> {
        if candidate.expected_disk != self.disk_baseline || candidate.revision > self.revision {
            return Err(EditorError::SaveConflict);
        }
        self.disk_baseline = candidate.text;
        Ok(())
    }

    fn next_revision(&self) -> Result<u64, EditorError> {
        self.revision
            .checked_add(1)
            .ok_or(EditorError::RevisionExhausted)
    }

    fn trim_history(&mut self) {
        while self.retained_history_bytes() > self.limits.history_bytes
            || self.undo.len() + self.redo.len() > self.limits.history_steps
        {
            if self.undo.pop_front().is_none() {
                self.redo.pop_front();
            }
        }
    }
}

#[cfg(test)]
mod tests;
