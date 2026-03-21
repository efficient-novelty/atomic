use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    emit("PEN_BUILD_PROFILE", env::var("PROFILE").ok(), "unknown");
    emit("PEN_TARGET_TRIPLE", env::var("TARGET").ok(), "unknown");
    emit(
        "PEN_TARGET_CPU",
        encoded_rustflags()
            .as_deref()
            .and_then(parse_target_cpu)
            .map(str::to_owned),
        "default",
    );
}

fn emit(key: &str, value: Option<String>, fallback: &str) {
    let value = value.unwrap_or_else(|| fallback.to_owned());
    println!("cargo:rustc-env={key}={value}");
}

fn encoded_rustflags() -> Option<String> {
    env::var("CARGO_ENCODED_RUSTFLAGS")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| env::var("RUSTFLAGS").ok())
}

fn parse_target_cpu(flags: &str) -> Option<&str> {
    let mut iter = flags
        .split(['\x1f', ' '])
        .filter(|token| !token.trim().is_empty());

    while let Some(token) = iter.next() {
        if let Some(value) = token.strip_prefix("-Ctarget-cpu=") {
            return Some(value);
        }
        if let Some(value) = token.strip_prefix("-Ctarget-cpu") {
            if let Some(value) = value.strip_prefix('=') {
                return Some(value);
            }
        }
        if token == "-C" {
            if let Some(next) = iter.next() {
                if let Some(value) = next.strip_prefix("target-cpu=") {
                    return Some(value);
                }
            }
        }
    }

    None
}
