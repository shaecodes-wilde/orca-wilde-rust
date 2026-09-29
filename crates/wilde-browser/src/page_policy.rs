use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    InvalidOrigin,
    InvalidUrl,
    WrongOwner,
    UnknownPage,
    DuplicatePage,
}

#[derive(Debug, Clone)]
pub struct NavigationPolicy {
    origins: Vec<String>,
}
impl NavigationPolicy {
    /// Deliberately narrow HTTPS grammar; this is not a general URL parser.
    pub fn https_only(origins: &[&str]) -> Result<Self, PolicyError> {
        let mut validated = Vec::new();
        for origin in origins {
            if strict_origin(origin)? != *origin {
                return Err(PolicyError::InvalidOrigin);
            }
            validated.push((*origin).into());
        }
        Ok(Self { origins: validated })
    }
    pub fn allows(&self, url: &str) -> bool {
        strict_origin(url).is_ok_and(|origin| self.origins.iter().any(|v| v == origin))
    }
}
fn strict_origin(url: &str) -> Result<&str, PolicyError> {
    if url.len() > 8192 || !url.is_ascii() || url.bytes().any(|b| b <= 32 || b == 127 || b == b'\\')
    {
        return Err(PolicyError::InvalidUrl);
    }
    let authority = url
        .strip_prefix("https://")
        .ok_or(PolicyError::InvalidUrl)?;
    let end = authority.find(['/', '?', '#']).unwrap_or(authority.len());
    let host = &authority[..end];
    if host.is_empty()
        || host.len() > 253
        || !host.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
    {
        return Err(PolicyError::InvalidOrigin);
    }
    Ok(&url[..8 + end])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Camera,
    Microphone,
    ClipboardRead,
    Geolocation,
    Notifications,
    Other,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Deny,
}
pub fn permission_decision(_permission: Permission) -> PermissionDecision {
    PermissionDecision::Deny
}

#[derive(Debug, Default)]
pub struct PageOwnership {
    owners: BTreeMap<String, String>,
}
impl PageOwnership {
    pub fn register(&mut self, owner: &str, page: &str) -> Result<(), PolicyError> {
        if owner.is_empty() || page.is_empty() {
            return Err(PolicyError::UnknownPage);
        }
        if self.owners.contains_key(page) {
            return Err(PolicyError::DuplicatePage);
        }
        self.owners.insert(page.into(), owner.into());
        Ok(())
    }
    /// Authorizes a resource owner to close one page; does not kill engine processes.
    pub fn release(&mut self, owner: &str, page: &str) -> Result<(), PolicyError> {
        match self.owners.get(page) {
            None => Err(PolicyError::UnknownPage),
            Some(actual) if actual != owner => Err(PolicyError::WrongOwner),
            Some(_) => {
                self.owners.remove(page);
                Ok(())
            }
        }
    }
    pub fn owned_pages(&self, owner: &str) -> Vec<String> {
        self.owners
            .iter()
            .filter(|(_, actual)| actual.as_str() == owner)
            .map(|(page, _)| page.clone())
            .collect()
    }
}

/// NavigateToString emits about:blank; allow exactly its first synthetic navigation.
#[derive(Debug)]
pub struct InitialSyntheticNavigation {
    pending: std::cell::Cell<bool>,
}
impl Default for InitialSyntheticNavigation {
    fn default() -> Self {
        Self {
            pending: std::cell::Cell::new(true),
        }
    }
}
impl InitialSyntheticNavigation {
    pub fn allows(&self, url: &str) -> bool {
        self.pending.replace(false) && url == "about:blank"
    }
}
