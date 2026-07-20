//! Rust bindings generated from `wit/data-provider.wit` via `wit-bindgen`.
//!
//! This module is the single generated artifact both the host and every plugin depend on
//! for the canonical schema types and the `data-provider` interface. See
//! `openspec/changes/define-league-data-contract/design.md` ("WIT interface lives in this
//! repo as the single source of truth").

#![allow(missing_docs, clippy::pedantic, clippy::nursery)]

wit_bindgen::generate!({
    world: "plugin",
    path: "wit",
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize, Clone, PartialEq],
    pub_export_macro: true,
    export_macro_name: "export",
});
