
use errors::*;
use regex::Regex;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

const ZIP_DEFAULT_FOLDER: &'static str = "zarc_zip_extracted";

#[derive(Debug)]
pub struct Archive {
    path: PathBuf,
    archive_type: ArchiveType,
}

#[derive(Debug)]
enum ArchiveType {
    GzipTarball,
    BzipTarball,
    Bzip,
    Rar,
    Gzip,
    Tarball,
    Zip,
    Zip7,
    XZip,
}

fn determine_archive_type(path: &Path) -> Option<ArchiveType> {
    let path_cow = path.to_string_lossy();
    let path_string: String = path_cow.into_owned();
    let rules = vec![(r"\.tar\.gz$", ArchiveType::GzipTarball),
                     (r"\.tgz$", ArchiveType::GzipTarball),
                     (r"\.tar.bz2$", ArchiveType::BzipTarball),
                     (r"\.tbz2$", ArchiveType::BzipTarball),
                     (r"\.bz2$", ArchiveType::Bzip),
                     (r"\.rar$", ArchiveType::Rar),
                     (r"\.gz$", ArchiveType::Gzip),
                     (r"\.Z$", ArchiveType::Gzip),
                     (r"\.tar$", ArchiveType::Tarball),
                     (r"\.zip$", ArchiveType::Zip),
                     (r"\.7z$", ArchiveType::Zip7),
                     (r"\.xz$", ArchiveType::XZip)];

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
            ArchiveType::BzipTarball => {
                vec!["tar".into(), "xjvf".into(), self.path.into_os_string()]
            },
            ArchiveType::Bzip => vec!["bunzip2".into(), self.path.into_os_string()],
            ArchiveType::Gzip => vec!["gzip".into(), "-dk".into(), self.path.into_os_string()],
            ArchiveType::Rar => vec!["unrar".into(), "x".into(), self.path.into_os_string()],
            ArchiveType::Tarball => vec!["tar".into(), "xvf".into(), self.path.into_os_string()],
            ArchiveType::Zip => {
                let mut new_dir_arg: OsString = OsString::from("-d");
                new_dir_arg.push(self.path
                    .file_stem()
                    .or_else(|| {
                        warn!("Unable to determine name of zip folder. Extracting into {}",
                              ZIP_DEFAULT_FOLDER);
                        Some(OsStr::from_bytes(ZIP_DEFAULT_FOLDER.as_bytes()))
                    })
                    .unwrap());
                vec!["unzip".into(), new_dir_arg, self.path.into_os_string()]
            },
            ArchiveType::Zip7 => vec!["7z".into(), "x".into(), self.path.into_os_string()],
            ArchiveType::XZip => vec!["xz".into(), "-dk".into(), self.path.into_os_string()],
        }
    }
}
