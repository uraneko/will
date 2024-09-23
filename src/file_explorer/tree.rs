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
pub enum Node<'a> {
    File {
        next: Option<&'a Node<'a>>,
        prev: Option<&'a Node<'a>>,
        value: PathBuf,
    },

    Dir {
        value: PathBuf,
        children: Vec<Box<Node<'a>>>,
        next: Option<&'a Node<'a>>,
        prev: Option<&'a Node<'a>>,
    },

    Root {
        children: Vec<Box<Node<'a>>>,
    },
}

impl<'a> Node<'a> {
    fn scan_root() -> Self {
        _ = fs::create_dir(PathBuf::from(TREE_ROOT));
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

    fn next<'b>(&'a self) -> &'b Option<Box<Node<'a>>>
    where
        'a: 'b,
    {
        match self {
            Self::File { next, .. } => next,
            Self::Dir { next, .. } => next,
            _ => return &None,
        }
    }

    pub(crate) fn next_ref<'b>(&'a self) -> Result<&'b Node<'a>, Error>
    where
        'a: 'b,
    {
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

    pub(crate) fn next_mut<'b>(&'a mut self) -> Result<&'b mut Node<'a>, Error>
    where
        'a: 'b,
    {
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

    fn prev<'b>(&'a self) -> &'b Option<Box<Node<'a>>>
    where
        'a: 'b,
    {
        match self {
            Self::File { prev, .. } => prev,
            Self::Dir { prev, .. } => prev,
            _ => return &None,
        }
    }

    pub(crate) fn prev_ref(&self) -> Result<&Node, Error> {
        let prev = match self {
            Self::File { prev, .. } => prev,
            Self::Dir { prev, .. } => prev,
            _ => return Err(Error::other("root has no siblings")),
        };

        if let Some(node) = prev {
            Ok(node)
        } else {
            Err(Error::other("prev node doesn't exist"))
        }
    }

    pub(crate) fn prev_mut(&mut self) -> Result<&mut Node, Error> {
        let prev = match self {
            Self::File { prev, .. } => prev,
            Self::Dir { prev, .. } => prev,
            _ => return Err(Error::other("root has no siblings")),
        };

        if let Some(ref mut node) = prev {
            Ok(node)
        } else {
            Err(Error::other("prev node doesn't exist"))
        }
    }
}

// TODO: forgot to implement next and prev
// prev and next are actually not needed
// because siblings are in a vector already
// recursive
fn scan_dir(p: PathBuf) -> Vec<Box<Node>> {
    let mut idx = 0;
    fs::read_dir(p)
        .unwrap()
        .filter(|ore| if let Ok(_) = ore { true } else { false })
        .map(|ore| ore.unwrap())
        .map(|e| e.path())
        .map(|p| {
            if p.is_file() {
                idx += 1;
                Node::file_from_path(p)
            } else if p.is_dir() {
                idx += 1;
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

// broken
fn link_dir(dir: &mut Vec<Box<Node>>) {
    match *dir[0] {
        Node::Dir { ref mut next, .. } | Node::File { ref mut next, .. } => *next = Some(&*dir[1]),
        _ => unreachable!("dont touch root"),
    }

    for idx in 1..dir.len() - 1 {
        match *dir[idx] {
            Node::Dir {
                ref mut next,
                ref mut prev,
                ..
            }
            | Node::File {
                ref mut next,
                ref mut prev,
                ..
            } => {
                *prev = Some(&*dir[idx - 1]);
                *next = Some(&*dir[idx + 1]);
            }
            _ => unreachable!("dont touch root"),
        }
    }

    match *dir[dir.len() - 1] {
        Node::Dir { ref mut prev, .. } | Node::File { ref mut prev, .. } => {
            *prev = Some(&*dir[dir.len() - 2])
        }
        _ => unreachable!("dont touch root"),
    }
}
