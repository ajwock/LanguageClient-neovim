use serde::{Deserialize, Serialize};

use crate::{Range, TextDocumentPositionParams, WorkDoneProgressOptions, WorkDoneProgressParams};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameParams {
    /// Text Document and Position fields
    #[serde(flatten)]
    pub text_document_position: TextDocumentPositionParams,

    /// The new name of the symbol. If the given name is not valid the
    /// request must return a [ResponseError](#ResponseError) with an
    /// appropriate message set.
    pub new_name: String,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOptions {
    /// Renames should be checked and tested before being executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_provider: Option<bool>,

    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCapability {
    /// Whether rename supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// Client supports testing for validity of rename operations before execution.
    ///
    /// since 3.12.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_support: Option<bool>,

    /// Client supports the default behavior result (`{ defaultBehavior: boolean }`).
    ///
    /// since 3.16.0
    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_support_default_behavior: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum PrepareRenameResponse {
    Range(Range),
    RangeWithPlaceholder { range: Range, placeholder: String },
    #[cfg(feature = "proposed")]
    DefaultBehavior { default_behavior: bool },
}
