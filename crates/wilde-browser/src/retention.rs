use std::collections::BTreeSet;

pub const DEFAULT_WARM_PAGE_BUDGET: usize = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsavedState {
    Clean,
    Dirty,
    Unknown,
}
#[derive(Debug, Clone, Default)]
pub struct Protection {
    pub visible: bool,
    pub media: bool,
    pub transfer: bool,
    pub automation_lease: bool,
    pub remote_viewer: bool,
    pub keep_alive: bool,
}
#[derive(Debug, Clone)]
pub struct WarmPage {
    pub page_id: String,
    pub unsaved: UnsavedState,
    pub protection: Protection,
    pub last_used_ms: u64,
    pub resident_since_ms: u64,
}
impl WarmPage {
    fn protected(&self) -> bool {
        self.unsaved != UnsavedState::Clean
            || self.protection.visible
            || self.protection.media
            || self.protection.transfer
            || self.protection.automation_lease
            || self.protection.remote_viewer
            || self.protection.keep_alive
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct EvictionPlan {
    pub evict: Vec<String>,
    /// Eligible warm pages whose eviction is deferred by cooldown or clock uncertainty.
    pub deferred_over_budget: usize,
    /// Protected residents are reported separately and are never eviction candidates.
    pub protected_resident_pages: usize,
    /// Informational total-resident pressure above the warm allowance after planned evictions.
    /// The warm allowance is not a hard total cap and never overrides page protection.
    pub remaining_resident_above_warm_budget: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum RetentionError {
    DuplicatePage,
    InvalidPage,
}
/// Inputs are resident pages only. Resource owners execute the plan after rechecking protections.
pub fn eviction_plan(
    pages: &[WarmPage],
    budget: usize,
    now_ms: u64,
    cooldown_ms: u64,
) -> Result<EvictionPlan, RetentionError> {
    let mut ids = BTreeSet::new();
    for page in pages {
        if page.page_id.is_empty() {
            return Err(RetentionError::InvalidPage);
        }
        if !ids.insert(&page.page_id) {
            return Err(RetentionError::DuplicatePage);
        }
    }
    let mut warm: Vec<_> = pages.iter().filter(|page| !page.protected()).collect();
    warm.sort_by(|a, b| {
        a.last_used_ms
            .cmp(&b.last_used_ms)
            .then(a.page_id.cmp(&b.page_id))
    });
    let protected_resident_pages = pages.len() - warm.len();
    let mut excess = warm.len().saturating_sub(budget);
    let mut evict = Vec::new();
    for page in warm {
        if excess == 0 {
            break;
        }
        // An expired lease remains protected until the owner reconciles it; clock rollback defers eviction.
        if now_ms
            .checked_sub(page.resident_since_ms)
            .is_some_and(|age| age >= cooldown_ms)
            && page.last_used_ms <= now_ms
        {
            evict.push(page.page_id.clone());
            excess -= 1;
        }
    }
    let remaining_resident_above_warm_budget = pages
        .len()
        .saturating_sub(evict.len())
        .saturating_sub(budget);
    Ok(EvictionPlan {
        evict,
        deferred_over_budget: excess,
        protected_resident_pages,
        remaining_resident_above_warm_budget,
    })
}
