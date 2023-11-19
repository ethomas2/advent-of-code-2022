use std::error::Error;
use std::fs;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Loc(i32, i32); // (row, col)
                      //
struct Board {
    width: i32,
    height: i32,
    boardarr: Vec<i32>,
    start: Loc,
    end: Loc,
}

impl Board {
    fn get(&self, loc: Loc) -> i32 {
        self.boardarr[(self.width * loc.0 + loc.1) as usize]
    }
}

mod bfs {
    use queues;
    use queues::IsQueue;
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::iter; // Must be here because items from buffer must be in scope
    use std::rc::Rc;

    // TODO: see if you can make this an iterator type NodeCollection<NodeType> = dyn
    // Iterator<Item = NodeType>;
    type NodeCollection<NodeType> = Vec<NodeType>;

    ////////// pub struct
    pub struct BfsIter<NodeType, C>
    where
        C: FnMut(&NodeType) -> NodeCollection<NodeType>,
        NodeType: Clone + Hash + Eq + Debug, // TODO: why do you need to be clone
    {
        queue: queues::Buffer<Rc<BfsNode<NodeType>>>,
        get_children: C,
        visited: HashSet<NodeType>,
    }

    ////////// pub struct
    #[derive(Clone)]
    pub struct BfsNode<NodeType> {
        pub val: NodeType,
        pub parent: Option<Rc<BfsNode<NodeType>>>,
    }

    ////////// impl Iterator
    impl<NodeType, C> Iterator for BfsIter<NodeType, C>
    where
        C: FnMut(&NodeType) -> NodeCollection<NodeType>,
        NodeType: Clone + Hash + Eq + Debug,
    {
        type Item = Rc<BfsNode<NodeType>>;

        fn next(&mut self) -> Option<Self::Item> {
            // println!("{:?}", self.visited);
            let head = self.queue.remove().ok()?;
            let children = (self.get_children)(&head.val);
            for child in children.into_iter() {
                let child_clone = child.clone(); // TODO: could just put RC<child> into the hash
                                                 // map?
                if !self.visited.contains(&child_clone) {
                    self.visited.insert(child_clone);
                    self.queue
                        .add(Rc::new(BfsNode {
                            val: child,
                            parent: Some(head.clone()),
                        }))
                        .expect("Unexpectedly could not add child to queue");
                }
            }
            Some(head)
        }
    }

    ////////// pub fn
    pub fn bfs<NodeType, C>(startnode: NodeType, get_children: C) -> BfsIter<NodeType, C>
    where
        C: FnMut(&NodeType) -> NodeCollection<NodeType>,
        NodeType: Clone + Hash + Eq + Debug,
    {
        // TODO: can you have things in the hashmap point to things in the queue?
        let visited: HashSet<NodeType> = iter::once(startnode.clone()).collect();
        let queue = {
            let mut buf = queues::Buffer::new(10000);
            buf.add(Rc::new(BfsNode {
                val: startnode,
                parent: None,
            }))
            .unwrap(); // panic if somehow queue size is full
            buf
        };
        return BfsIter {
            queue,
            get_children,
            visited,
        };
    }

    #[cfg(test)]
    mod test_bfs {
        use super::*;
        #[test]
        fn test_simple() {
            // i guess this doesn't actually test anything. It just runs the code and makes sure it
            // doesn't panic or endless loop
            let iter = bfs(1, |&x| {
                if x < 10 {
                    vec![2 * x, 2 * x + 1]
                } else {
                    vec![]
                }
            });
            assert_eq!(
                iter.map(|x| *x).collect::<Vec<_>>(),
                (1..=19).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_does_not_revisit_parent() {
            // cyclc grooup
            let iter = bfs(0, |&x| vec![(x + 1) % 5]);
            let x: Vec<i32> = iter.map(|x| *x).take(10).collect();
            assert_eq!(x, (0..5).collect::<Vec<_>>());

            // grid from (0, 0) to (4, 4)
            let n = 5;
            let iter = bfs((0, 0), |&(r, c)| {
                vec![(r + 1, c), (r, c + 1)]
                    .into_iter()
                    .filter(|&(r, c)| r < n && c < n)
                    .collect()
            });
            let mut result = iter.map(|x| *x).collect::<Vec<_>>();
            result.sort();
            assert_eq!(result, {
                let mut tuples = Vec::new();
                for i in 0..n {
                    for j in 0..n {
                        tuples.push((i, j));
                    }
                }
                tuples.sort();
                tuples
            });
        }
    }
}

fn parse(content: &str) -> Result<Board, String> {
    let height = content.lines().count() as i32;
    let width = content.lines().nth(0).expect("no rows").len() as i32;

    let chars_iter = || content.lines().flat_map(|line| line.chars());

    let mut boardarr: Vec<i32> = Vec::new();

    for ch in chars_iter() {
        boardarr.push(match ch {
            'S' => 0,
            'E' => 25,
            _ if ch.is_ascii_lowercase() => ((ch as u32) - 97) as i32,
            _ => return Err(format!("unexpected input {}", ch)),
        });
    }

    let idx = chars_iter()
        .enumerate()
        .find_map(|(idx, ch)| match ch {
            'S' => Some(idx),
            _ => None,
        })
        .ok_or("couldn't find 'S' in input")?;
    let start: Loc = Loc((idx as i32 / width) as i32, (idx as i32 % width) as i32);

    let idx = chars_iter()
        .enumerate()
        .find_map(|(idx, ch)| match ch {
            'E' => Some(idx),
            _ => None,
        })
        .ok_or("Couldn't find 'E' in input")?;
    let end: Loc = Loc((idx as i32 / width) as i32, (idx as i32 % width) as i32);
    Ok(Board {
        width,
        height,
        boardarr,
        start,
        end,
    })
}
fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/d12/input")?;
    let board = parse(content.as_str())?;

    let mut nodeiter = bfs::bfs(board.start, |&currentloc @ Loc(r, c)| {
        let newlocs = vec![Loc(r + 1, c), Loc(r - 1, c), Loc(r, c + 1), Loc(r, c - 1)]
            .into_iter()
            // TODO: why don't i have to borrow board?
            .filter(|&Loc(r, c)| 0 <= r && r < board.height && 0 <= c && c < board.width)
            .filter(|&newloc| board.get(newloc) <= board.get(currentloc) + 1)
            .collect::<Vec<_>>();

        newlocs
    });

    let found_node = nodeiter
        .find(|node| node.val == board.end)
        .expect("Could not find a path to end node");

    let n = std::iter::successors(Some(found_node), |node| node.parent.clone()).count();

    println!("{:?}", n - 1);

    // let mut sequence: Vec<Rc<bfs::BfsNode<Loc>>> =
    //     std::iter::successors(Some(found_node), |node| node.parent.clone()).collect();

    // sequence.reverse();
    // let sequence = sequence;
    // for (i, node) in sequence.iter().enumerate() {
    //     println!("{:?}", (i, node.val));
    // }

    // let mut n = 0;

    // loop {
    //     match found_node.parent.clone() {
    //         Some(parent) => {
    //             found_node = parent;
    //             n += 1;
    //         }
    //         None => {
    //             break;
    //         }
    //     }
    // }

    // n
    // };

    // println!("{}", nsteps);

    // for loc in nodeiter {
    //     if loc.val == board.end {
    //         println!("found it");
    //         break;
    //     }
    // }
    Ok(())
}
