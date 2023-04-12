use std::cell::RefCell;
use std::rc::Rc;
// use std::assert_matches::assert_matches;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut it = s.split(' ');
        if it.next() != Some("$") {
            return Err(String::from("Expected $ as first character of line"));
        }
        let cmd = match it.next() {
            Some("ls") => Ok(Self::Ls),
            Some("cd") => match it.next() {
                Some(x) => Ok(Self::Cd(x)),
                None => Err(String::from("No token after cd")),
            },
            Some(x) => Err(String::from(format!(
                "Unexpected token {x} after str",
                x = x
            ))),
            None => Err(String::from("No token after $")),
        };

        if let Err(_) = cmd {
            return cmd;
        } else if let Some(x) = it.next() {
            return Err(String::from(format!(
                "Unexpected tokens after finished parsing lines {x}",
                x = x
            )));
        }

        return cmd;
    }
}

// TODO: why do i need the lifetime parameter here?. Oh you always need it for enums and structs. Never elided
// Ans: It's not that the str must outlive the filenode (although that is true), it's that it
// refernces some other piece of text and that other text must outlive the filenode
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FileNode<'a> {
    Dir(&'a str),
    File { size: usize, name: &'a str },
}

#[derive(Debug, PartialEq, Eq)]
enum ParsedLine<'a> {
    Command(Command<'a>),
    FileNode(FileNode<'a>),
}

impl<'a> TryFrom<&'a str> for FileNode<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut it = s.split(' ');
        let cmd = match it.next() {
            Some("dir") => match it.next() {
                Some(dirname) => Ok(FileNode::Dir(dirname)),
                None => Err(String::from("No token after dir")),
            },
            Some(size) => match it.next() {
                Some(name) => {
                    let s = size.parse::<usize>().map_err(|e| e.to_string())?;
                    Ok(FileNode::File { size: s, name })
                }
                None => Err(String::from("Must have two entries")),
            },
            None => Err(String::from("Empty string cannot be parsed to a FileNode")),
        }?;

        if let Some(_) = it.next() {
            return Err(String::from("Extra values after the last expected token"));
        }

        Ok(cmd)
    }
}

type NodeHandle<'a> = Rc<RefCell<Node<'a>>>;

#[derive(Debug)]
enum Node<'a> {
    File {
        name: &'a str,
        size: usize,
        parent: Option<NodeHandle<'a>>,
    },
    Dir {
        name: &'a str,
        // TODO: does it make sense to have hashmap &str? should it be hashmap str?
        children: HashMap<&'a str, NodeHandle<'a>>,
        parent: Option<NodeHandle<'a>>,
    },
}

impl<'a> Node<'a> {
    fn parent(&self) -> Option<NodeHandle<'a>> {
        let p = match self {
            Node::File { parent, .. } => parent,
            Node::Dir { parent, .. } => parent,
        };
        return p.clone();
    }
    fn name(&self) -> &'a str {
        match self {
            Node::File { name, .. } => name,
            Node::Dir { name, .. } => name,
        }
    }
}

fn put<'a, 'b>(parent: NodeHandle<'a>, fnode: FileNode<'b>) -> NodeHandle<'a>
where
    'b: 'a,
{
    let newnode: NodeHandle = match fnode {
        FileNode::Dir(name) => Rc::new(RefCell::new(Node::Dir {
            name,
            children: HashMap::new(),
            parent: Some(parent.clone()),
        })),
        FileNode::File { size, name } => Rc::new(RefCell::new(Node::File {
            name,
            size,
            parent: Some(parent.clone()),
        })),
    };

    if let Node::Dir { children, .. } = &mut *parent.clone().borrow_mut() {
        match fnode {
            FileNode::Dir(dirname) => {
                children.insert(dirname, newnode.clone());
            }
            FileNode::File { name, .. } => {
                children.insert(name, newnode.clone());
            }
        }
    }
    return newnode;
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d07/input")?;
    let mut parsed_lines = content
        .lines()
        .map(|line| {
            if let Ok(cmd) = Command::try_from(line) {
                let x: Result<ParsedLine, String> = Ok(ParsedLine::Command(cmd));
                return x;
            };
            return Ok(ParsedLine::FileNode(FileNode::try_from(line)?));
        })
        .peekable();

    let first = parsed_lines.next().ok_or("No first value")??;
    assert_eq!(first, ParsedLine::Command(Command::Cd("/")));
    let root = Rc::new(RefCell::new(Node::Dir {
        name: "/",
        children: HashMap::new(),
        parent: None,
    }));
    let mut current_node = root.clone();

    let it = &mut parsed_lines;
    while let Some(_pl) = it.next() {
        let pl = _pl?;
        match pl {
            ParsedLine::Command(Command::Cd("..")) => {
                current_node = current_node.clone().borrow().parent().unwrap();
            }
            ParsedLine::Command(Command::Cd(dirname)) => {
                current_node = put(current_node.clone(), FileNode::Dir(dirname));
            }
            ParsedLine::Command(Command::Ls) => {
                while let Some(Ok(ParsedLine::FileNode(fnode))) = it.peek() {
                    // push fnode to State
                    put(current_node.clone(), *fnode);
                    it.next();
                }
            }
            ParsedLine::FileNode(_) => return Err("got file node not preceeded by ls")?,
        }
    }

    let nodes_and_sizes = dfs(root.clone());
    let big_ones = nodes_and_sizes
        .iter()
        .filter(|(handle, size)| matches!(*handle.borrow(), Node::Dir { .. }) && *size <= 100000);

    let total: usize = big_ones.map(|(_, size)| size).sum();
    println!("{}", total); // part 1
                           //
    let root_size = nodes_and_sizes[0].1;
    println!("root_size {}", root_size);
    let unused_space = 70000000 - root_size;
    let amt_to_free = 30000000 - unused_space;
    println!("amt_to_free {}", amt_to_free);
    let big_enough = nodes_and_sizes
        .iter()
        .filter_map(|(handle, size)| match *handle.borrow() {
            Node::Dir { .. } if *size >= amt_to_free => Some(*size),
            _ => None,
        })
        .min()
        .unwrap();
    println!("{}", big_enough); // part 1

    Ok(())
}

fn dfs(r: NodeHandle) -> Vec<(NodeHandle, usize)> {
    let mut results: Vec<(NodeHandle, usize)> = vec![];
    let mut total: usize = 0;
    match *r.borrow() {
        Node::Dir { ref children, .. } => {
            for child in children.values() {
                let child_results = dfs(child.clone());
                let (_, t) = &child_results[0];
                total += t;
                results.extend(child_results);
            }
            results.insert(0, (r.clone(), total));
        }
        Node::File { size, .. } => {
            results.push((r.clone(), size));
        }
    }
    results
}
