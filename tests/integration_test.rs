//! Test public APIs

use std::{env, path::PathBuf};

use oxc_resolver::{Resolution, ResolveContext, ResolveOptions, Resolver};

fn dir() -> PathBuf {
    env::current_dir().unwrap()
}

fn resolve(specifier: &str) -> Resolution {
    let path = dir();
    Resolver::new(ResolveOptions::default()).resolve(path, specifier).unwrap()
}

#[test]
fn clone() {
    let resolution = resolve("./tests/package.json");
    assert_eq!(resolution.clone(), resolution);
}

#[test]
fn debug() {
    let resolution = resolve("./tests/package.json");
    let s = format!("{:?}", resolution);
    assert!(!s.is_empty());
}

#[test]
fn eq() {
    let resolution = resolve("./tests/package.json");
    assert_eq!(resolution, resolution);
}

#[test]
fn package_json() {
    let resolution = resolve("./tests/package.json");
    assert!(resolution.package_json().is_some_and(|json| json.raw_json().is_object()));
}

#[test]
fn clear_cache() {
    let resolver = Resolver::new(ResolveOptions::default());
    resolver.clear_cache(); // exists
}

#[test]
fn options() {
    let resolver = Resolver::new(ResolveOptions::default());
    let options = resolver.options();
    assert!(!format!("{options:?}").is_empty());
}

#[test]
fn debug_resolver() {
    let resolver = Resolver::new(ResolveOptions::default());
    assert!(!format!("{resolver:?}").is_empty());
}

#[test]
fn dependencies() {
    let path = dir();
    let mut ctx = ResolveContext::default();
    let _ = Resolver::new(ResolveOptions::default()).resolve_with_context(
        path,
        "./tests/package.json",
        &mut ctx,
    );
    assert!(!ctx.file_dependencies.is_empty());
    assert!(ctx.missing_dependencies.is_empty());
}
