use errors::*;
use regex::Regex;
use std::borrow::Cow;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Archive {
    path: PathBuf,
    archive_type: ArchiveType,
}

#[derive(Debug)]
enum ArchiveType {
    GzipTarball,
}

fn determine_archive_type(path: &Path) -> Option<ArchiveType> {
    let path_cow = path.to_string_lossy();
    let path_string: String = path_cow.into_owned();
    let rules = vec![(r"\.tar\.gz$", ArchiveType::GzipTarball),
                     (r"\.tgz$", ArchiveType::GzipTarball)];
    for (re_str, archive_type) in rules {
        let re = Regex::new(re_str).unwrap();
        debug!("Checking '{}' against '{:?}'", path_string, re);
        if re.is_match(&path_string) {
            info!("Matched '{}' => {:?}", path_string, archive_type);
            return Some(archive_type);
        }
    }
    None
}

fn resolve_path<'a, S>(raw: S) -> Result<PathBuf>
    where S: Into<Cow<'a, Path>>
{
    let user_path = raw.into();
    fs::canonicalize(&user_path).chain_err(|| ErrorKind::UnknownInputFile(user_path.into_owned()))
}

impl Archive {
    pub fn from_user_path<'a, S>(raw: S) -> Result<Archive>
        where S: Into<Cow<'a, Path>>
    {
        let full_path = resolve_path(raw)?;
        debug!("Resolved full path to archive: {}", full_path.display());
        determine_archive_type(&full_path)
            // TODO This clone shouldn't be necessary
            .ok_or_else(|| ErrorKind::UnrecognizedArchive(full_path.clone()).into())
            .map(|archive_type| {
                Archive {
                    path: full_path,
                    archive_type: archive_type,
                }
            })
    }

    pub fn extract_cmd(self) -> Vec<OsString> {
        match self.archive_type {
            ArchiveType::GzipTarball => {
                vec!["tar".into(), "xzvf".into(), self.path.into_os_string()]
            },
        }
    }
}
