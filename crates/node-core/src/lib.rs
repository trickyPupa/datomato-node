//! Minimal application boundary; wire types must not define the whole domain.
/// Check a peer's advertised bootstrap version before future application work.
pub fn check_peer_version(version: u32) -> Result<(), datomato_protocol::UnsupportedVersion> {
    datomato_protocol::check_version(version)
}

/// Advertised protocol version; distinct from the product package version.
pub fn protocol_version() -> u32 {
    datomato_protocol::SUPPORTED_PROTOCOL_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_compatibility_is_enforced_at_application_boundary() {
        assert!(check_peer_version(1).is_ok());
        for version in [0, 2, u32::MAX] {
            let error = check_peer_version(version).expect_err("must reject incompatible peer");
            assert_eq!(error.received, version);
            assert!(error.to_string().contains("unsupported protocol version"));
        }
    }
}
