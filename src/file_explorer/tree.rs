use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Error;
use std::path::{Path, PathBuf};

// NOTE: how to get the path of the tree root??
// write it here as a const: YES
// write it in the frontend js as a const: NO

const TREE_ROOT: &str = "explorer/";

pub(crate) fn tree() -> Node {
    Node::scan_root()
}

// #[derive(PartialEq, Eq, Hash)]
pub(crate) enum Node {
    File {
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
        value: PathBuf,
    },

    Dir {
        value: PathBuf,
        children: Vec<Box<Node>>,
        next: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    },

    Root {
        children: Vec<Box<Node>>,
    },
}

impl Node {
    fn scan_root() -> Self {
        Node::Root {
            children: scan_dir(PathBuf::from(TREE_ROOT)),
        }
    }

    fn file_from_path(p: PathBuf) -> Self {
        Self::File {
            next: None,
            prev: None,
            value: p,
        }
    }

    fn dir_from_path(p: PathBuf) -> Self {
        Self::Dir {
            next: None,
            prev: None,
            value: p,
            children: vec![],
        }
    }

    fn next(&self) -> &Option<Box<Node>> {
        match self {
            Self::File { next, .. } => next,
            Self::Dir { next, .. } => next,
            _ => return &None,
        }
    }

    fn prev(&self) -> &Option<Box<Node>> {
        match self {
            Self::File { prev, .. } => prev,
            Self::Dir { prev, .. } => prev,
            _ => return &None,
        }
    }

    pub fn next_ref(&self) -> Result<&Node, Error> {
        let next = match self {
            Self::File { next, .. } => next,
            Self::Dir { next, .. } => next,
            _ => return Err(Error::other("root has no siblings")),
        };

        if let Some(node) = next {
            Ok(node)
        } else {
            Err(Error::other("next node doesn't exist"))
        }
    }

    pub fn next_mut(&mut self) -> Result<&mut Node, Error> {
        let next = match self {
            Self::File { next, .. } => next,
            Self::Dir { next, .. } => next,
            _ => return Err(Error::other("root has no siblings")),
        };

        if let Some(ref mut node) = next {
            Ok(node)
        } else {
            Err(Error::other("next node doesn't exist"))
        }
    }
}

// recursive
fn scan_dir(p: PathBuf) -> Vec<Box<Node>> {
    fs::read_dir(p)
        .unwrap()
        .filter(|ore| if let Ok(_) = ore { true } else { false })
        .map(|ore| ore.unwrap())
        .map(|e| e.path())
        .map(|p| {
            if p.is_file() {
                Node::file_from_path(p)
            } else if p.is_dir() {
                Node::dir_from_path(p)
            } else {
                panic!("dir entry '{:?}' was neither dir nor file", p)
            }
        })
        .map(|mut node| {
            if let Node::Dir {
                ref mut children,
                ref value,
                ..
            } = node
            {
                *children = scan_dir(value.to_path_buf());
            }
            node
        })
        .map(|node| Box::new(node))
        .collect::<Vec<Box<Node>>>()
}
