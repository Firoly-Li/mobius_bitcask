use std::collections::HashSet;

#[derive(Clone,Debug)]
pub struct OldFileService {
    old_files: HashSet<String>
}