//! Settings for tweaking assists.
//!
//! The fun thing here is `SnippetCap` -- this type can only be created in this
//! module, and we use to statically check that we only produce snippet
//! assists if we are allowed to.

use ide_db::{imports::insert_use::InsertUseConfig, SnippetCap};

use crate::AssistKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssistConfig {
    pub snippet_cap: Option<SnippetCap>,
    pub allowed: Option<Vec<AssistKind>>,
    pub insert_use: InsertUseConfig,
    pub prefer_no_std: bool,
    pub prefer_prelude: bool,
    pub assist_emit_must_use: bool,
    // If set to `Some(...)`, we just get the only assist corresponding to this diagnostic code.
    pub specified_diagnostic_code: Option<String>,
}
