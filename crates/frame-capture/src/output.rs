use std::{
    borrow::Borrow,
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::CaptureRoute;
use crate::ids::StateIdValueValidator;
use koruma::Validate as _;

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureOutputName(String);

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureOutputStem(String);

#[derive(
    Clone, Debug, derive_more::AsRef, derive_more::Deref, derive_more::From, Eq, Hash, PartialEq,
)]
#[as_ref(forward)]
#[deref(forward)]
pub struct CaptureOutputRoot(PathBuf);

#[derive(derive_more::AsRef, Clone, Debug, derive_more::Deref, Eq, Hash, PartialEq)]
#[as_ref(forward)]
#[deref(forward)]
pub struct CaptureOutputPath(PathBuf);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CaptureOutputPathError {
    #[error("capture output must not be empty")]
    Empty,
    #[error("capture output stem must not include .png")]
    Extension,
    #[error("capture output name must end with .png")]
    MissingExtension,
    #[error("capture output must be a file stem, not a path")]
    PathComponent,
    #[error("capture output must be valid Unicode")]
    NotUnicode,
}

impl CaptureOutputName {
    /// Creates a PNG output file name.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when `value` is empty, path-like, or
    /// does not end with `.png`.
    pub fn new(value: impl Into<String>) -> Result<Self, CaptureOutputPathError> {
        Self::try_from(value.into())
    }

    pub fn from_stem(stem: &CaptureOutputStem) -> Self {
        Self(format!("{}.png", stem.as_str()))
    }

    pub fn current() -> Self {
        Self::from_stem(&CaptureOutputStem::current())
    }

    pub fn reference() -> Self {
        Self::from_stem(&CaptureOutputStem::reference())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl CaptureOutputStem {
    /// Creates a PNG output file stem.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when `value` is empty, path-like, or
    /// already includes `.png`.
    pub fn new(value: impl Into<String>) -> Result<Self, CaptureOutputPathError> {
        Self::try_from(value.into())
    }

    pub fn current() -> Self {
        Self("current".to_owned())
    }

    pub fn reference() -> Self {
        Self("reference".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl CaptureOutputRoot {
    pub fn new(value: impl Into<PathBuf>) -> Self {
        Self(value.into())
    }

    pub fn captures() -> Self {
        Self(PathBuf::from("captures"))
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl CaptureOutputPath {
    /// Creates a PNG output path.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when the path is empty, has no file
    /// name, has a non-Unicode file name, or does not end with `.png`.
    pub fn new(value: impl Into<PathBuf>) -> Result<Self, CaptureOutputPathError> {
        Self::try_from(value.into())
    }

    /// Builds an output path from a root directory, route, and PNG file name.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when the joined path is not a valid
    /// PNG output path.
    pub fn for_name<R: CaptureRoute>(
        root: impl AsRef<Path>,
        route: R,
        output: &CaptureOutputName,
    ) -> Result<Self, CaptureOutputPathError> {
        Self::new(root.as_ref().join(route.id()).join(output.as_str()))
    }

    /// Builds an output path from a root directory, route, and PNG file stem.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when the joined path is not a valid
    /// PNG output path.
    pub fn for_stem<R: CaptureRoute>(
        root: impl AsRef<Path>,
        route: R,
        output: &CaptureOutputStem,
    ) -> Result<Self, CaptureOutputPathError> {
        Self::for_name(root, route, &CaptureOutputName::from_stem(output))
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl Borrow<str> for CaptureOutputName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<CaptureOutputName> for String {
    fn from(value: CaptureOutputName) -> Self {
        value.into_string()
    }
}

impl FromStr for CaptureOutputName {
    type Err = CaptureOutputPathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for CaptureOutputName {
    type Error = CaptureOutputPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_output_name(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CaptureOutputName {
    type Error = CaptureOutputPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Borrow<str> for CaptureOutputStem {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<CaptureOutputStem> for String {
    fn from(value: CaptureOutputStem) -> Self {
        value.into_string()
    }
}

impl FromStr for CaptureOutputStem {
    type Err = CaptureOutputPathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for CaptureOutputStem {
    type Error = CaptureOutputPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_output_stem(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CaptureOutputStem {
    type Error = CaptureOutputPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Borrow<Path> for CaptureOutputRoot {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl fmt::Display for CaptureOutputRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_path().display().fmt(formatter)
    }
}

impl From<CaptureOutputRoot> for PathBuf {
    fn from(value: CaptureOutputRoot) -> Self {
        value.into_path_buf()
    }
}

impl FromStr for CaptureOutputRoot {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(value))
    }
}

impl Serialize for CaptureOutputRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_path().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CaptureOutputRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        PathBuf::deserialize(deserializer).map(Self::new)
    }
}

impl Default for CaptureOutputRoot {
    fn default() -> Self {
        Self::captures()
    }
}

impl Borrow<Path> for CaptureOutputPath {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl From<CaptureOutputPath> for PathBuf {
    fn from(value: CaptureOutputPath) -> Self {
        value.into_path_buf()
    }
}

impl PartialEq<PathBuf> for CaptureOutputPath {
    fn eq(&self, other: &PathBuf) -> bool {
        self.as_path() == other.as_path()
    }
}

impl PartialEq<CaptureOutputPath> for PathBuf {
    fn eq(&self, other: &CaptureOutputPath) -> bool {
        self.as_path() == other.as_path()
    }
}

impl FromStr for CaptureOutputPath {
    type Err = CaptureOutputPathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<PathBuf> for CaptureOutputPath {
    type Error = CaptureOutputPathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        validate_output_path(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&Path> for CaptureOutputPath {
    type Error = CaptureOutputPathError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::try_from(value.to_path_buf())
    }
}

impl Serialize for CaptureOutputPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_path().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CaptureOutputPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Self::new(path).map_err(serde::de::Error::custom)
    }
}

/// Builds a route-local PNG output path from a file stem.
///
/// # Errors
///
/// Returns [`CaptureOutputPathError`] when the joined path is not a valid PNG
/// output path.
pub fn capture_output_path_for_stem<R: CaptureRoute>(
    root: impl AsRef<Path>,
    route: R,
    output: &CaptureOutputStem,
) -> Result<PathBuf, CaptureOutputPathError> {
    CaptureOutputPath::for_stem(root, route, output).map(CaptureOutputPath::into_path_buf)
}

/// Builds a route-local PNG output path from a file name.
///
/// # Errors
///
/// Returns [`CaptureOutputPathError`] when the joined path is not a valid PNG
/// output path.
pub fn capture_output_path_for_name<R: CaptureRoute>(
    root: impl AsRef<Path>,
    route: R,
    output: &CaptureOutputName,
) -> Result<PathBuf, CaptureOutputPathError> {
    CaptureOutputPath::for_name(root, route, output).map(CaptureOutputPath::into_path_buf)
}

fn validate_output_name(value: &str) -> Result<(), CaptureOutputPathError> {
    if value.is_empty() {
        return Err(CaptureOutputPathError::Empty);
    }
    if !StateIdValueValidator.validate(&value) {
        return Err(CaptureOutputPathError::PathComponent);
    }
    if !value.ends_with(".png") {
        return Err(CaptureOutputPathError::MissingExtension);
    }

    Ok(())
}

fn validate_output_stem(value: &str) -> Result<(), CaptureOutputPathError> {
    if value.is_empty() {
        return Err(CaptureOutputPathError::Empty);
    }
    if !StateIdValueValidator.validate(&value) {
        return Err(CaptureOutputPathError::PathComponent);
    }
    if value.ends_with(".png") {
        return Err(CaptureOutputPathError::Extension);
    }

    Ok(())
}

fn validate_output_path(value: &Path) -> Result<(), CaptureOutputPathError> {
    if value.as_os_str().is_empty() {
        return Err(CaptureOutputPathError::Empty);
    }

    let Some(file_name) = value.file_name() else {
        return Err(CaptureOutputPathError::PathComponent);
    };
    let Some(file_name) = file_name.to_str() else {
        return Err(CaptureOutputPathError::NotUnicode);
    };
    if !file_name.ends_with(".png") {
        return Err(CaptureOutputPathError::MissingExtension);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn output_names_and_stems_cover_named_conversions() {
        let stem = CaptureOutputStem::try_from("review").unwrap();
        let stem_borrowed: &str = stem.borrow();
        assert_eq!(stem_borrowed, "review");
        assert_eq!(String::from(stem.clone()), "review");
        assert_eq!("review".parse::<CaptureOutputStem>().unwrap(), stem);
        assert_eq!(CaptureOutputStem::current().as_str(), "current");
        assert_eq!(CaptureOutputStem::reference().into_string(), "reference");

        let name = CaptureOutputName::from_stem(&stem);
        let name_borrowed: &str = name.borrow();
        assert_eq!(name_borrowed, "review.png");
        assert_eq!(String::from(name.clone()), "review.png");
        assert_eq!("review.png".parse::<CaptureOutputName>().unwrap(), name);
        assert_eq!(CaptureOutputName::try_from("review.png").unwrap(), name);
        assert_eq!(CaptureOutputName::current().as_str(), "current.png");
        assert_eq!(
            CaptureOutputName::reference().into_string(),
            "reference.png"
        );
    }

    #[test]
    fn output_name_and_stem_validation_reports_each_contract_error() {
        assert_eq!(
            CaptureOutputName::new(""),
            Err(CaptureOutputPathError::Empty)
        );
        assert_eq!(
            CaptureOutputName::new("."),
            Err(CaptureOutputPathError::PathComponent)
        );
        assert_eq!(
            CaptureOutputName::new("review"),
            Err(CaptureOutputPathError::MissingExtension)
        );
        assert_eq!(
            CaptureOutputStem::new(""),
            Err(CaptureOutputPathError::Empty)
        );
        assert_eq!(
            CaptureOutputStem::new(".."),
            Err(CaptureOutputPathError::PathComponent)
        );
        assert_eq!(
            CaptureOutputStem::new("review.png"),
            Err(CaptureOutputPathError::Extension)
        );
    }

    #[test]
    fn output_roots_support_path_and_serde_conversions() {
        let root = CaptureOutputRoot::default();
        let borrowed: &Path = root.borrow();
        assert_eq!(borrowed, Path::new("captures"));
        assert_eq!(root.to_string(), "captures");
        assert_eq!(PathBuf::from(root.clone()), PathBuf::from("captures"));
        assert_eq!(
            "reference".parse::<CaptureOutputRoot>().unwrap().as_path(),
            Path::new("reference")
        );
        assert_eq!(serde_json::to_string(&root).unwrap(), r#""captures""#);
        assert_eq!(
            serde_json::from_str::<CaptureOutputRoot>(r#""reference""#)
                .unwrap()
                .into_path_buf(),
            PathBuf::from("reference")
        );
    }

    #[test]
    fn output_paths_support_path_and_serde_conversions() {
        let path = CaptureOutputPath::try_from(Path::new("captures/review.png")).unwrap();
        let borrowed: &Path = path.borrow();
        assert_eq!(borrowed, Path::new("captures/review.png"));
        assert_eq!(path, PathBuf::from("captures/review.png"));
        assert_eq!(PathBuf::from("captures/review.png"), path);
        assert_eq!(
            "captures/review.png".parse::<CaptureOutputPath>().unwrap(),
            path
        );
        assert_eq!(
            serde_json::to_string(&path).unwrap(),
            r#""captures/review.png""#
        );
        assert_eq!(
            serde_json::from_str::<CaptureOutputPath>(r#""captures/review.png""#).unwrap(),
            path
        );
        assert_eq!(PathBuf::from(path), PathBuf::from("captures/review.png"));
    }

    #[test]
    fn output_paths_reject_invalid_file_paths() {
        assert_eq!(
            CaptureOutputPath::new(""),
            Err(CaptureOutputPathError::Empty)
        );
        assert_eq!(
            CaptureOutputPath::new("/"),
            Err(CaptureOutputPathError::PathComponent)
        );
        assert_eq!(
            CaptureOutputPath::new("capture.jpg"),
            Err(CaptureOutputPathError::MissingExtension)
        );
        assert!(serde_json::from_str::<CaptureOutputPath>(r#""capture.jpg""#).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn output_paths_reject_non_unicode_file_names() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

        let path = PathBuf::from(OsString::from_vec(vec![0xff]));
        assert_eq!(
            CaptureOutputPath::new(path),
            Err(CaptureOutputPathError::NotUnicode)
        );
    }
}
