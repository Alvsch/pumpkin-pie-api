use cargo_lock::Lockfile;

pub fn emit_plugin_api_git_hash() {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();

    let hash = lockfile
        .packages
        .iter()
        .find(|p| p.name.as_str() == "pumpkin-plugin-api")
        .and_then(|p| p.source.as_ref())
        .and_then(|s| s.precise())
        .unwrap_or("unknown");

    let short_hash = &hash[..7];

    println!("cargo:rustc-env=PLUGIN_API_GIT_HASH={hash}");
    println!("cargo:rustc-env=PLUGIN_API_GIT_HASH_SHORT={short_hash}");
    println!("cargo:rerun-if-changed=Cargo.lock");
}