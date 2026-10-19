//! A contracts crate with a socket dependency would be a review failure
//! without a test catching it. Same guard the other crates carry.

const BANNED: &[&str] = &[
    "reqwest",
    "hyper",
    "tokio",
    "ureq",
    "curl",
    "native-tls",
    "openssl",
    "rustls",
    "isahc",
    "attohttpc",
    "surf",
    "tungstenite",
    "quinn",
];

#[test]
fn no_network_capable_crates_in_lock() {
    let lock = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"))
        .expect("Cargo.lock must be committed so this guard can read it");
    for banned in BANNED {
        let marker = format!("name = \"{banned}\"");
        assert!(
            !lock.contains(&marker),
            "network-capable crate `{banned}` must not enter Cargo.lock"
        );
    }
}
