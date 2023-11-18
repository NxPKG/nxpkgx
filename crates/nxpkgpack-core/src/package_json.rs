use std::{fmt::Write, ops::Deref};

use anyhow::Result;
use serde_json::Value as JsonValue;
use nxpkg_tasks::{debug::ValueDebugFormat, trace::TraceRawVcs, ReadRef, Vc};
use nxpkg_tasks_fs::{FileContent, FileJsonContent, FileSystemPath};

use super::issue::Issue;
use crate::issue::{IssueExt, StyledString};

/// PackageJson wraps the parsed JSON content of a `package.json` file. The
/// wrapper is necessary so that we can reference the [FileJsonContent]'s inner
/// [serde_json::Value] without cloning it.
#[derive(PartialEq, Eq, ValueDebugFormat, TraceRawVcs)]
pub struct PackageJson(ReadRef<FileJsonContent>);

impl Deref for PackageJson {
    type Target = JsonValue;
    fn deref(&self) -> &Self::Target {
        match &*self.0 {
            FileJsonContent::Content(json) => json,
            _ => unreachable!("PackageJson is guaranteed to hold Content"),
        }
    }
}

#[nxpkg_tasks::value(transparent, serialization = "none")]
pub struct OptionPackageJson(Option<PackageJson>);

/// Reads a package.json file (if it exists). If the file is unparseable, it
/// emits a useful [Issue] pointing to the invalid location.
#[nxpkg_tasks::function]
pub async fn read_package_json(path: Vc<FileSystemPath>) -> Result<Vc<OptionPackageJson>> {
    let read = path.read_json().await?;
    match &*read {
        FileJsonContent::Content(_) => Ok(OptionPackageJson(Some(PackageJson(read))).cell()),
        FileJsonContent::NotFound => Ok(OptionPackageJson(None).cell()),
        FileJsonContent::Unparseable(e) => {
            let mut message = "package.json is not parseable: invalid JSON: ".to_string();
            if let FileContent::Content(content) = &*path.read().await? {
                let text = content.content().to_str()?;
                e.write_with_content(&mut message, &text)?;
            } else {
                write!(message, "{}", e)?;
            }
            PackageJsonIssue {
                error_message: message,
                path,
            }
            .cell()
            .emit();
            Ok(OptionPackageJson(None).cell())
        }
    }
}

/// Reusable Issue struct representing any problem with a `package.json`
#[nxpkg_tasks::value(shared)]
pub struct PackageJsonIssue {
    pub path: Vc<FileSystemPath>,
    pub error_message: String,
}

#[nxpkg_tasks::value_impl]
impl Issue for PackageJsonIssue {
    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        Vc::cell("Error parsing package.json file".to_string())
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("parse".to_string())
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.path
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        StyledString::Text(self.error_message.clone()).cell()
    }
}
