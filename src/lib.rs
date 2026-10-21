//! Oxidt's Topcoat UI registry.
//!
//! Add it as a dependency, then copy components into an app with
//! `topcoat ui add <name> --registry oxidt-ui`. The theme in `theme/oxidt.css`
//! supplies the classes the components use; see the README.
//!
//! Component sources compile as test modules so changes are checked before
//! installation.

#[cfg(test)]
#[allow(dead_code)]
mod components;
