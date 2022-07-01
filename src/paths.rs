use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

use crate::error::ParsePathError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paths {
    paths: Vec<PathBuf>,
}

impl Paths {
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            paths: vec!["LICENSE".into()],
        }
    }
}

impl Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut comma = false;
        for path in self.paths() {
            if comma {
                write!(f, ",")?;
            }
            write!(f, "{}", path.display())?;
            comma = true;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a OsStr> for Paths {
    type Error = ParsePathError;

    fn try_from(s: &OsStr) -> Result<Paths, ParsePathError> {
        if s.is_empty() {
            return Err(ParsePathError);
        }

        #[cfg(windows)]
        return try_paths_from_str_windows(s);

        #[cfg(unix)]
        return try_paths_from_str_unix(s);
    }
}

#[cfg(windows)]
fn try_paths_from_str_windows(s: &OsStr) -> Result<Paths, ParsePathError> {
    let mut result = Vec::new();
    let mut pathbuf = Vec::new();

    let mut comma = [0; 2];
    let comma = &*','.encode_utf16(&mut comma);

    for wc in s.encode_wide() {
        if [wc] == comma {
            if pathbuf.is_empty() {
                return Err(ParsePathError);
            }
            result.push(PathBuf::from(OsString::from_wide(&pathbuf)));
            pathbuf.clear();
        } else {
            pathbuf.push(wc);
        }
    }

    if !pathbuf.is_empty() {
        result.push(PathBuf::from(OsString::from_wide(&pathbuf)));
    }

    Ok(Paths { paths: result })
}

#[cfg(unix)]
pub fn try_paths_from_str_unix(s: &OsStr) -> Result<Paths, ParsePathError> {
    let result = s
        .as_bytes()
        .split(b',')
        .map(|path| {
            if path.is_empty() {
                Err(ParsePathError)
            } else {
                Ok(PathBuf::from(OsStr::from_bytes(path)))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Paths { paths: result })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_1() {
        assert_eq!(
            Paths {
                paths: vec!["x".into()],
            }
            .to_string(),
            "x",
        );
    }

    #[test]
    fn display_2() {
        assert_eq!(
            Paths {
                paths: vec!["x".into(), "y".into()],
            }
            .to_string(),
            "x,y",
        );
    }

    #[test]
    fn display_3() {
        assert_eq!(
            Paths {
                paths: vec!["x".into(), "y".into(), "z".into()],
            }
            .to_string(),
            "x,y,z",
        );
    }

    #[test]
    fn display_lossy() {
        #[cfg(windows)]
        {
            assert_eq!(
                Paths {
                    paths: vec![
                        PathBuf::from(OsString::from_wide(&[0x0078, 0xd800])),
                        PathBuf::from(OsString::from_wide(&[0x0079, 0xd800])),
                        PathBuf::from(OsString::from_wide(&[0x007a, 0xd800])),
                    ],
                }
                .to_string(),
                "x\u{fffd},y\u{fffd},z\u{fffd}",
            );
        }

        #[cfg(unix)]
        {
            assert_eq!(
                Paths {
                    paths: vec![
                        PathBuf::from(OsStr::from_bytes(&[b'x', 0xff])),
                        PathBuf::from(OsStr::from_bytes(&[b'y', 0xff])),
                        PathBuf::from(OsStr::from_bytes(&[b'z', 0xff])),
                    ],
                }
                .to_string(),
                "x\u{fffd},y\u{fffd},z\u{fffd}",
            );
        }
    }

    #[test]
    fn conversion_1() {
        assert_eq!(
            Paths::try_from("x".as_ref()),
            Ok(Paths {
                paths: vec!["x".into()],
            }),
        );
    }

    #[test]
    fn conversion_2() {
        assert_eq!(
            Paths::try_from("x,y".as_ref()),
            Ok(Paths {
                paths: vec!["x".into(), "y".into()],
            }),
        );
    }

    #[test]
    fn conversion_3() {
        assert_eq!(
            Paths::try_from("x,y,z".as_ref()),
            Ok(Paths {
                paths: vec!["x".into(), "y".into(), "z".into()],
            }),
        );
    }

    #[test]
    fn conversion_lossy() {
        #[cfg(windows)]
        {
            assert_eq!(
                Paths::try_from(&*OsString::from_wide(&[
                    0x0078, 0xd800, 0x002c, 0x0079, 0xd800, 0x002c, 0x007a, 0xd800
                ])),
                Ok(Paths {
                    paths: vec![
                        PathBuf::from(OsString::from_wide(&[0x0078, 0xd800])),
                        PathBuf::from(OsString::from_wide(&[0x0079, 0xd800])),
                        PathBuf::from(OsString::from_wide(&[0x007a, 0xd800])),
                    ],
                }),
            );
        }

        #[cfg(unix)]
        {
            assert_eq!(
                Paths::try_from(OsStr::from_bytes(&[
                    b'x', 0xff, b',', b'y', 0xff, b',', b'z', 0xff
                ])),
                Ok(Paths {
                    paths: vec![
                        PathBuf::from(OsStr::from_bytes(&[b'x', 0xff])),
                        PathBuf::from(OsStr::from_bytes(&[b'y', 0xff])),
                        PathBuf::from(OsStr::from_bytes(&[b'z', 0xff])),
                    ],
                }),
            );
        }
    }
}
