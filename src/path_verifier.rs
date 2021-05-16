use std::collections::HashSet;
use std::path::Path;

/// Struct that verifies [`Path`] objects against provided arguments
///
/// Holds a [`HashSet`] of folders to exclude and a [`HashSet`] of files the user is searching for.
#[derive(Debug)]
pub struct PathVerifier {
    restricted_folders: HashSet<String>,
    search_files: HashSet<String>,
}

impl PathVerifier {
    /// Add a new file name to search for
    pub fn add_search_file(&mut self, file_name: &str) -> Self {
        self.search_files.insert(file_name.to_string());
        Self {
            restricted_folders: self.restricted_folders.clone(),
            search_files: self.search_files.clone(),
        }
    }

    /// Add a new folder to exclude from search
    pub fn add_restricted_folder(&mut self, folder_name: &str) -> Self {
        self.restricted_folders.insert(folder_name.to_string());
        Self {
            restricted_folders: self.restricted_folders.clone(),
            search_files: self.search_files.clone(),
        }
    }

    /// Test if the [`Path`] is valid per user constaints
    ///
    /// The [`Path`] must not have any folder component which is in the excluded folders and
    /// the file name must be in the set of names to search for.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// // The path contains "__MACOSX" which is excluded by default
    /// let file_path = Path::new("__MACOSX/index.js");
    /// let verifier = PathVerifier::default().add_search_file("index.js");
    /// assert!(!verifier.verify(file_path));
    /// ```
    pub fn verify(&self, path: &Path) -> bool {
        let mut pieces = path.components().map(|comp| comp.as_os_str());
        pieces.all(|comp| !self.restricted_folders.contains(comp.to_str().unwrap()))
            && self
                .search_files
                .contains(path.file_name().unwrap().to_str().unwrap())
    }
}

impl Default for PathVerifier {
    fn default() -> Self {
        Self {
            restricted_folders: ["__MACOSX".to_owned(), "node_modules".to_owned()]
                .iter()
                .cloned()
                .collect(),
            search_files: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_verifier_denies_when_file_in_restricted() {
        let file_path = Path::new("__MACOSX/index.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_accepts_file_nested() {
        let file_path = Path::new("foo/bar/index.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_accepts_file_bare() {
        let file_path = Path::new("index.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_folder_named_search() {
        let file_path = Path::new("foo/bar/index.js/incorrect.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_file_nested() {
        let file_path = Path::new("foo/bar/baz.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_file_bare() {
        let file_path = Path::new("baz.js");
        let verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }
}
