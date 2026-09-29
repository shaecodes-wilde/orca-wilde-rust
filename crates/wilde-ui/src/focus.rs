use crate::layout::{Region, SceneLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Forward,
    Backward,
}

pub struct FocusRoute;

impl FocusRoute {
    // Computes intent only; the main-thread scene owns native focus effects.
    pub fn step(
        current: Option<Region>,
        direction: FocusDirection,
        layout: &SceneLayout,
    ) -> Option<Region> {
        const ORDER: [Region; 4] = [
            Region::Sidebar,
            Region::NativeInput,
            Region::Terminal,
            Region::Browser,
        ];
        let start = current.and_then(|region| ORDER.iter().position(|item| *item == region));
        for offset in 1..=ORDER.len() {
            let index = match (start, direction) {
                (Some(start), FocusDirection::Forward) => (start + offset) % ORDER.len(),
                (Some(start), FocusDirection::Backward) => {
                    (start + ORDER.len() - offset) % ORDER.len()
                }
                (None, FocusDirection::Forward) => offset - 1,
                (None, FocusDirection::Backward) => ORDER.len() - offset,
            };
            let region = ORDER[index];
            if layout.bounds(region).is_some() {
                return Some(region);
            }
        }
        None
    }
}
