use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{config, download, humble_api};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    UnionUsizeRanges(String),
    ExtractFilenameFromUrl(String),
    BulkFormat(String),
    ChoicePeriod(String),
    // -- Modules
    #[from]
    Config(#[serde_as(as = "DisplayFromStr")] config::ConfigError),
    #[from]
    HumbleApi(#[serde_as(as = "DisplayFromStr")] humble_api::ApiError),
    #[from]
    Download(#[serde_as(as = "DisplayFromStr")] download::DownloadError),
    // -- Externals
    #[from]
    StdIo(#[serde_as(as = "DisplayFromStr")] std::sync::Arc<std::io::Error>),
    #[from]
    ByteParsing(#[serde_as(as = "DisplayFromStr")] byte_unit::ParseError),
    #[from]
    ToSocketAddrIo(#[serde_as(as = "DisplayFromStr")] std::io::Error),
    #[from]
    Reqwest(#[serde_as(as = "DisplayFromStr")] reqwest::Error),
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
