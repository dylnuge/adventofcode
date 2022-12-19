// Advent of Code 2022 Day 7
// Structures and functions to handle representing a fake filesystem

use std::collections::HashMap;

// Arguably for this simple case it's easier to use a traditional tree, but
// in Rust, having a tree with self-referencial parents and mutable data
// requires some ugly runtime borrow checking with RefCells.
#[derive(Debug)]
pub struct FileSystem {
    // This is a map of absoloute paths to FsEntry objects. We could also use
    // numeric IDs, but since we don't have to handle anything like links,
    // this should be sufficient (and guarantee that we don't have duplicate
    // entries at the same path). We assume there will always be a root entry
    // (stored at "/") which is our entry point into the file system.
    pub nodes: HashMap<String, FsEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FsEntry {
    Directory(DirectoryEntry),
    File(FileEntry),
}

#[derive(Debug)]
pub struct DirectoryEntry {
    // The path of a directory should include a trailing slash (arbitrary,
    // but we need consistency here since they're being used as unique IDs)
    pub path: String,
    // Relative name (no slashes). The name of the root is the empty string.
    pub name: String,
    // These strings are path IDs into FileSystem
    pub children: Vec<String>,
    pub parent: Option<String>,
}

// A derived eq would check all elements, including the children vec, but
// we only need to check path equality since it's a unique ID
impl PartialEq for DirectoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for DirectoryEntry {}

#[derive(Debug)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub size: usize,
}

impl PartialEq for FileEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FileEntry {}

impl FileSystem {
    pub fn mkdir(&mut self, path: &str) -> () {
        assert!(path.chars().last().unwrap() == '/');
        if !self.nodes.contains_key(path) {
            let dirs: Vec<&str> = path.split('/').collect();
            let name = dirs[dirs.len() - 2];
            let parent_path = get_path_from_relative(path, "..");

            // Add the listing to the parent
            let mut parent_node = &mut self.nodes.get_mut(&parent_path).unwrap();
            if let FsEntry::Directory(ref mut parent_dir) = parent_node {
                parent_dir.children.push(path.to_owned());
            } else {
                panic!("Parent directory doesn't exist!");
            }

            // Add the directory entry to the filesystem
            let dir_entry = DirectoryEntry {
                path: path.to_owned(),
                name: name.to_owned(),
                parent: Some(parent_path),
                children: Vec::new(),
            };
            self.nodes
                .insert(path.to_owned(), FsEntry::Directory(dir_entry));
        } else {
            // noop for now
        }
    }

    pub fn mkfile(&mut self, dir_path: &str, file_name: &str, size: usize) -> () {
        // Add the listing to the parent directory
        let path = format!("{}{}", dir_path, file_name);
        let mut dir_node = &mut self.nodes.get_mut(dir_path).unwrap();
        if let FsEntry::Directory(ref mut dir_entry) = dir_node {
            dir_entry.children.push(path.to_owned());
        } else {
            panic!("Parent directory doesn't exist!");
        }

        // Add the file entry to the filesystem
        let file_entry = FileEntry {
            path: path.to_owned(),
            name: file_name.to_owned(),
            size,
        };
        self.nodes
            .insert(path.to_owned(), FsEntry::File(file_entry));
    }

    pub fn get_dir_size(&self, dir_path: &DirectoryEntry) -> usize {
        let mut size: usize = 0;
        for child in &dir_path.children {
            let fs_node = self.nodes.get(child).unwrap();
            match fs_node {
                FsEntry::File(file_entry) => {
                    size += file_entry.size;
                }
                FsEntry::Directory(directory_entry) => {
                    size += self.get_dir_size(directory_entry);
                }
            }
        }

        size
    }

    pub fn get_root(&self) -> &DirectoryEntry {
        let root_node = self.nodes.get("/").unwrap();
        match root_node {
            FsEntry::Directory(dir_entry) => dir_entry,
            FsEntry::File(_) => panic!("Bad root!"),
        }
    }
}

// Initalization function for our file system; creates a FileSystem with an
// empty root directory
pub fn make_fs() -> FileSystem {
    let root_dir = FsEntry::Directory(DirectoryEntry {
        path: "/".to_owned(),
        name: "".to_owned(),
        children: Vec::new(),
        parent: None,
    });
    let mut node_map: HashMap<String, FsEntry> = HashMap::new();
    node_map.insert("/".to_owned(), root_dir);

    FileSystem { nodes: node_map }
}

// The utils in std::path can handle this too, IDK why I felt like writing
// a little thing instead, but I did
pub fn get_path_from_relative(curr_path: &str, rel_path: &str) -> String {
    assert!(curr_path.chars().last().unwrap() == '/');
    // Our system doesn't currently support nested paths, e.g.  things like
    // "dir1/dir2" or "../../dir3," so the path must be one of "/", "..", or the
    // name of a directory (and also, will always be relative unless it is "/")
    match rel_path {
        "/" => "/".to_owned(),
        ".." => {
            let dirs: Vec<&str> = curr_path.split('/').collect();
            dirs[..dirs.len() - 2].join("/") + "/"
        }
        dir_name => format!("{}{}/", curr_path, dir_name),
    }
}

// Library tests; I didn't bother with a cargofile
fn main() {
    let path1 = get_path_from_relative("/", "foo");
    let path2 = get_path_from_relative("/foo/bar/baz/", "..");
    let path3 = get_path_from_relative("/foo/bar/baz/", "foobar");
    let path4 = get_path_from_relative("/foo/bar/baz/", "/");
    assert_eq!(path1, "/foo/");
    assert_eq!(path2, "/foo/bar/");
    assert_eq!(path3, "/foo/bar/baz/foobar/");
    assert_eq!(path4, "/");
}
