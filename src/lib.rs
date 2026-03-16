/// Library crate — exposes all layers for integration tests and tooling.
pub mod adapter;
/// Core types and functionality for this crate.
pub mod core;
/// Utilities and functionality for the PAL subsystem.
pub mod pal;
/// Shared types, utilities, and functionality used across the crate.
pub mod shared;
/// Manages application state, configuration, and lifecycle.
pub mod state;
/// MCP server UI layer.
pub mod ui;
