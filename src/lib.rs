//! `cargo-wizard` is a Cargo subcommand that can apply preconfigured templates to your Cargo.toml manifest.
//!
//! Command-line usage:
//! ```bash
//! cargo wizard apply <profile> <template>
//! ```
//!
//! You can also use this crate as a library, although it probably won't be very useful.

pub use templates::*;
pub use workspace::{CargoWorkspace, parse_workspace};
pub use workspace::manifest::CargoManifest;
pub use workspace::manifest::resolve_manifest_path;
pub use workspace::manifest::TomlProfileTemplate;

mod templates;
mod toml;
mod workspace;
