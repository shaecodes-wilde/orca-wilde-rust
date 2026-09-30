// Admission watchdog for the pinned vte 0.15 state transitions; it stores no bytes.
#[derive(Clone, Copy, Default)]
pub(super) struct ControlBudget {
    state: State,
    pending: usize,
}

#[derive(Clone, Copy, Default)]
enum State {
    #[default]
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiIgnore,
    DcsEntry,
    DcsParam,
    DcsIntermediate,
    DcsIgnore,
    DcsPassthrough,
    Osc,
    SosPmApc,
}

impl ControlBudget {
    pub(super) fn admit(&mut self, bytes: &[u8]) -> Result<(), super::TerminalError> {
        for &byte in bytes {
            if !matches!(self.state, State::Ground) {
                if self.pending == super::MAX_CONTROL_SEQUENCE_BYTES {
                    return Err(super::TerminalError::ControlSequenceLimit);
                }
                self.pending += 1;
            }
            let redundant_escape = matches!(self.state, State::Escape) && byte == 0x1b;
            self.advance(byte);
            if matches!(self.state, State::Ground) {
                self.pending = 0;
            } else if byte == 0x1b && !redundant_escape {
                self.pending = 1;
            }
        }
        Ok(())
    }

    fn anywhere(&mut self, byte: u8) {
        match byte {
            0x18 | 0x1a => self.state = State::Ground,
            0x1b => self.state = State::Escape,
            _ => {}
        }
    }

    fn advance(&mut self, byte: u8) {
        use State::*;
        match self.state {
            Ground => {
                if byte == 0x1b {
                    self.state = Escape;
                }
            }
            Escape => match byte {
                0x20..=0x2f => self.state = EscapeIntermediate,
                0x50 => self.state = DcsEntry,
                0x58 | 0x5e | 0x5f => self.state = SosPmApc,
                0x5b => self.state = CsiEntry,
                0x5d => self.state = Osc,
                0x30..=0x7e => self.state = Ground,
                _ => self.anywhere(byte),
            },
            EscapeIntermediate => match byte {
                0x30..=0x7e => self.state = Ground,
                _ => self.anywhere(byte),
            },
            CsiEntry | DcsEntry => {
                let dcs = matches!(self.state, DcsEntry);
                match byte {
                    0x20..=0x2f => {
                        self.state = if dcs {
                            DcsIntermediate
                        } else {
                            CsiIntermediate
                        };
                    }
                    0x30..=0x3f => self.state = if dcs { DcsParam } else { CsiParam },
                    0x40..=0x7e => self.state = if dcs { DcsPassthrough } else { Ground },
                    _ => self.anywhere(byte),
                }
            }
            CsiParam | DcsParam => {
                let dcs = matches!(self.state, DcsParam);
                match byte {
                    0x20..=0x2f => {
                        self.state = if dcs {
                            DcsIntermediate
                        } else {
                            CsiIntermediate
                        };
                    }
                    0x3c..=0x3f => self.state = if dcs { DcsIgnore } else { CsiIgnore },
                    0x40..=0x7e => self.state = if dcs { DcsPassthrough } else { Ground },
                    _ => self.anywhere(byte),
                }
            }
            CsiIntermediate | DcsIntermediate => {
                let dcs = matches!(self.state, DcsIntermediate);
                match byte {
                    0x30..=0x3f => self.state = if dcs { DcsIgnore } else { CsiIgnore },
                    0x40..=0x7e => self.state = if dcs { DcsPassthrough } else { Ground },
                    _ => self.anywhere(byte),
                }
            }
            CsiIgnore => match byte {
                0x40..=0x7e => self.state = Ground,
                _ => self.anywhere(byte),
            },
            DcsIgnore | SosPmApc => self.anywhere(byte),
            DcsPassthrough => match byte {
                0x9c => self.state = Ground,
                _ => self.anywhere(byte),
            },
            Osc => match byte {
                0x07 => self.state = Ground,
                _ => self.anywhere(byte),
            },
        }
    }

    #[cfg(test)]
    pub(super) fn pending_bytes(&self) -> usize {
        self.pending
    }
}
