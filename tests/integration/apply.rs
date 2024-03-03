use crate::utils::{init_cargo_project, OutputExt};

#[test]
fn apply_explicit_manifest_path() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    let manifest_path = "crates/inner/Cargo.toml";
    project.file(
        manifest_path,
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"
"#,
    );
    project
        .cmd(&[
            "apply",
            "dev",
            "fast-compile",
            "--manifest-path",
            manifest_path,
        ])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read(manifest_path), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn resolve_workspace_root() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;
    project.file(
        "bar/Cargo.toml",
        r#"
[package]
name = "bar"
version = "0.1.0"
edition = "2021"
"#,
    );
    project.file("bar/src/lib.rs", "");
    project.manifest(
        r#"
[workspace]
members = ["bar"]
"#,
    );

    project
        .cmd(&["apply", "dev", "fast-compile"])
        .cwd(&project.path("bar"))
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [workspace]
    members = ["bar"]

    [profile.dev]
    debug = 0
"###);

    Ok(())
}

#[test]
fn apply_missing_builtin() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project
        .cmd(&["apply", "dev", "fast-compile"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_existing_builtin() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.dev]
debug = 1
"#,
    );

    project
        .cmd(&["apply", "dev", "fast-compile"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_missing_custom() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project
        .cmd(&["apply", "custom1", "fast-compile"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom1]
    inherits = "dev"
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_existing_custom() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.custom1]
inherits = "dev"
debug = 1
"#,
    );

    project
        .cmd(&["apply", "custom1", "fast-compile"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"
    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom1]
    inherits = "dev"
    debug = 0
    "###);

    Ok(())
}

#[test]
fn apply_existing_keep_formatting() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project.manifest(
        r#"
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[profile.dev]

lto =      "thin"

debug = 1   # Foo

codegen-units    = 10
"#,
    );

    project
        .cmd(&["apply", "dev", "fast-compile"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.dev]

    lto =      "thin"

    debug = 0   # Foo

    codegen-units    = 10
    "###);

    Ok(())
}

#[test]
fn apply_fast_runtime_template() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project
        .cmd(&["apply", "custom", "fast-runtime"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom]
    inherits = "release"
    lto = true
    codegen-units = 1
    panic = "abort"
    "###);

    Ok(())
}

#[test]
fn apply_min_size_template() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;

    project
        .cmd(&["apply", "custom", "min-size"])
        .run()?
        .assert_ok();
    insta::assert_snapshot!(project.read_manifest(), @r###"

    [package]
    name = "foo"
    version = "0.1.0"
    edition = "2021"

    [profile.custom]
    inherits = "release"
    debug = 0
    strip = true
    opt-level = "z"
    lto = true
    codegen-units = 1
    panic = "abort"
    "###);

    Ok(())
}
