use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Struct that verifies [`Path`] objects against provided arguments
///
/// Holds a [`HashSet`] of folders to exclude and a [`HashMap`] of files the user is searching for.
#[derive(Debug)]
pub struct PathVerifier {
    restricted_folders: HashSet<String>,
    search_files: HashMap<String, bool>,
}

impl PathVerifier {
    /// Add a new file name to search for
    pub fn add_search_file(&mut self, file_name: &str) -> Self {
        self.search_files.insert(file_name.to_string(), false);
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
    /// let mut verifier = PathVerifier::default().add_search_file("index.js");
    /// assert!(!verifier.verify(file_path));
    /// ```
    pub fn verify(&mut self, path: &Path) -> bool {
        let mut pieces = path.components().map(|comp| comp.as_os_str());
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if *self.search_files.get(&file_name).unwrap_or(&true) {
            return false;
        }
        let valid = pieces.all(|comp| !self.restricted_folders.contains(comp.to_str().unwrap()))
            && self.search_files.contains_key(&file_name);
        self.search_files.insert(file_name, valid);
        return valid;
    }

    /// Resets the progress of a PathVerifier
    pub fn reset(&mut self) -> () {
        for key in self.search_files.clone().keys() {
            self.search_files.insert(key.to_string(), false);
        }
    }

    pub fn print_progress(&self, folder_name: &String) -> () {
        let not_found = self
            .search_files
            .clone()
            .into_iter()
            .filter_map(|(k, v)| if v { None } else { Some(k) })
            .collect::<Vec<String>>();
        for name in not_found {
          error!("{} was not found in {}", name, folder_name);
        }
    }
}

impl Default for PathVerifier {
    fn default() -> Self {
        Self {
            restricted_folders: ["__MACOSX".to_owned(), "node_modules".to_owned()]
                .iter()
                .cloned()
                .collect(),
            search_files: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_verifier_denies_when_file_in_restricted() {
        let file_path = Path::new("__MACOSX/index.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_accepts_file_nested() {
        let file_path = Path::new("foo/bar/index.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_accepts_file_bare() {
        let file_path = Path::new("index.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_folder_named_search() {
        let file_path = Path::new("foo/bar/index.js/incorrect.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_file_nested() {
        let file_path = Path::new("foo/bar/baz.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }

    #[test]
    fn path_verifier_denies_file_bare() {
        let file_path = Path::new("baz.js");
        let mut verifier = PathVerifier::default().add_search_file("index.js");
        assert!(!verifier.verify(file_path));
    }
}
