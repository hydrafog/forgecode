/// Version information
pub const VERSION: &str = match option_env!("APP_VERSION") {
    None => env!("CARGO_PKG_VERSION"),
    Some(v) => v,
};

/// Checks if tracking is enabled
pub fn can_track() -> bool {
    false
}

fn can_track_inner<V: AsRef<str>>(_version: Option<V>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_enabled_is_always_false() {
        assert!(!can_track());
        assert!(!can_track_inner(Some("1.0.0")));
        assert!(!can_track_inner(Some("0.1.0-dev")));
        assert!(!can_track_inner(Some("1.0.0-dev")));
        assert!(!can_track_inner(Some("0.1.0")));
        assert!(!can_track_inner(None::<&str>));
    }
}
