use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Error;
use std::path::{Path, PathBuf};

// NOTE: how to get the path of the tree root??
// write it here as a const: YES
// write it in the frontend js as a const: NO

const TREE_ROOT: &str = "explorer/";

pub fn tree() -> Node {
    Node::scan_root()
}

#[derive(Debug, Clone)]
pub enum Node {
    File {
        value: PathBuf,
    },

    Dir {
        value: PathBuf,
        children: Vec<Box<Node>>,
    },

    Root {
        tree: Vec<Box<Node>>,
        node: Vec<usize>,
    },
}

type OBNode = Option<Box<Node>>;

impl Node {
    fn scan_root() -> Self {
        _ = fs::create_dir(PathBuf::from(TREE_ROOT));
        Node::Root {
            tree: vec![],
            node: vec![],
        }
    }

    fn file_from_path(p: PathBuf) -> Self {
        Self::File { value: p }
    }

    fn dir_from_path(p: PathBuf) -> Self {
        Self::Dir {
            value: p,
            children: vec![],
        }
    }

    fn dir_with_children(p: PathBuf) -> Self {
        Self::Dir {
            children: scan_dir(&p),
            value: p,
        }
    }

    fn new(p: PathBuf) -> Self {
        if p.is_dir() {
            Self::dir_from_path(p)
        } else {
            Self::file_from_path(p)
        }
    }

    // file or dir variant method
    pub(crate) fn value(&self) -> Option<&PathBuf> {
        match self {
            Self::File { value } => Some(value),
            Self::Dir { value, .. } => Some(value),
            Self::Root { .. } => None,
        }
    }

    // root variant method
    pub(crate) fn find_dir(&self) -> Result<&[Box<Node>], NodeErr> {
        let Self::Root { .. } = self else {
            return Err(NodeErr::NotRoot);
        };

        self.tree_walk()
    }

    // root variant method
    pub(crate) fn prev(&mut self) -> Result<usize, NodeErr> {
        let Self::Root { ref mut node, .. } = self else {
            return Err(NodeErr::NotRoot);
        };

        match node.pop() {
            Some(idx) => Ok(idx),
            None => Err(NodeErr::IndexEmpty),
        }
    }

    // root variant method
    pub(crate) fn nextn(&mut self, n: usize) -> Result<usize, NodeErr> {
        let Self::Root { ref mut node, .. } = self else {
            return Err(NodeErr::NotRoot);
        };

        node.push(n);

        Ok(n)
    }

    // root variant method
    pub(crate) fn index_len(&self) -> Option<usize> {
        let Self::Root { node, .. } = self else {
            return None;
        };

        Some(node.len())
    }

    // root variant method
    pub(crate) fn tree_walk(&self) -> Result<&[Box<Node>], NodeErr> {
        let Self::Root { ref tree, node } = self else {
            return Err(NodeErr::NotRoot);
        };

        let mut node = node.into_iter().rev().map(|n| *n).collect::<Vec<usize>>();
        let mut temp = &tree[node.pop().unwrap()];

        return loop {
            temp = temp.tree_step(node.pop().unwrap()).unwrap();

            if node.is_empty() {
                break temp.children();
            }
        };
    }

    // variant dir method
    fn children(&self) -> Result<&[Box<Node>], NodeErr> {
        let Self::Dir { ref children, .. } = self else {
            return Err(NodeErr::NotADir);
        };

        Ok(children)
    }

    fn tree_step(&self, idx: usize) -> Result<&Box<Node>, NodeErr> {
        let Self::Dir { ref children, .. } = self else {
            return Err(NodeErr::NotADir);
        };

        let Self::Dir { .. } = *children[idx] else {
            return Err(NodeErr::NotADir);
        };

        Ok(&children[idx])
    }

    pub(crate) fn path_ref(&self, p: &PathBuf) -> Result<Option<&Box<Node>>, NodeErr> {
        let dir = match self {
            Self::Root { ref tree, .. } => tree,
            Self::Dir { ref children, .. } => children,
            Self::File { .. } => return Err(NodeErr::RootOrDirExpected),
        };

        Ok(dir.into_iter().find(|node| {
            if let Self::Dir { ref value, .. } = ***node {
                value == p
            } else if let Self::File { ref value, .. } = ***node {
                value == p
            } else {
                panic!("found root node where it shouldnt have been");
            }
        }))
    }

    pub(crate) fn path_mut(&mut self, p: &PathBuf) -> Result<Option<&mut Box<Node>>, NodeErr> {
        let dir = match self {
            Self::Root { ref mut tree, .. } => tree,
            Self::Dir {
                ref mut children, ..
            } => children,
            Self::File { .. } => return Err(NodeErr::RootOrDirExpected),
        };

        Ok(dir.iter_mut().find(|node| {
            if let Self::Dir { ref value, .. } = ***node {
                value == p
            } else if let Self::File { ref value, .. } = ***node {
                value == p
            } else {
                panic!("found root node where it shouldnt have been");
            }
        }))
    }
}

#[derive(Debug)]
pub(crate) enum NodeErr {
    NotAFile,
    DirNotFound,
    RootOrDirExpected,
    NotADir,
    NotRoot,
    IndexEmpty,
}

// TODO: forgot to implement next and prev
// prev and next are actually not needed
// because siblings are in a vector already
pub fn scan_dir(p: &PathBuf) -> Vec<Box<Node>> {
    fs::read_dir(p)
        .unwrap()
        .filter(|ore| if let Ok(_) = ore { true } else { false })
        .map(|ore| ore.unwrap())
        .map(|e| e.path())
        .map(|p| {
            if p.is_dir() {
                Box::new(Node::dir_with_children(p))
            } else {
                Box::new(Node::file_from_path(p))
            }
        })
        .collect::<Vec<Box<Node>>>()
}

// recursive
// pub fn link_dir(p: PathBuf) -> Node {
//     let mut dir = scan_dir(&p);
//     if dir.is_empty() {
//         return Node::new(p);
//     }
//
//     let item = dir.remove(0);
//     let mut node0 = if item.is_dir() {
//         link_dir(item)
//     } else {
//         Node::new(item)
//     };
//
//     while !dir.is_empty() {
//         let item = dir.remove(0);
//
//         let mut node1 = if item.is_dir() {
//             link_dir(item)
//         } else {
//             Node::new(item)
//         };
//         node0 = node0.next(Box::new(node1.clone()));
//         node1 = node1.prev(Box::new(node0.clone()));
//         node0 = node1;
//     }
//
//     node0
// }
