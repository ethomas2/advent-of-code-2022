use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;
use std::ops::Add;
use std::rc::Rc;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn char(&self) -> char {
        match self {
            Self::Left => '<',
            Self::Right => '>',
            Self::Up => '^',
            Self::Down => 'v',
        }
    }
}
type Blizzard = Direction;

#[derive(Clone)]
enum GridSpace {
    Wall,
    Space(Vec<Blizzard>),
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            _ => Err(format!("Cannot convert char \"{}\" into Direction", ch)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Location(isize, isize);

impl Add<Direction> for Location {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self {
        let r = self.0;
        let c = self.1;
        match rhs {
            Direction::Up => (r - 1, c).into(),
            Direction::Right => (r, c + 1).into(),
            Direction::Down => (r + 1, c).into(),
            Direction::Left => (r, c - 1).into(),
        }
    }
}

impl<T, U> From<(T, U)> for Location
where
    T: Into<isize>,
    U: Into<isize>,
{
    fn from((x, y): (T, U)) -> Self {
        Location(x.into(), y.into())
    }
}

#[derive(Clone)]
struct Grid {
    grid: HashMap<Location, GridSpace>,
    height: usize,
    width: usize,
}

impl<'a> From<&'a str> for Grid {
    fn from(content: &'a str) -> Self {
        let grid = content
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars().enumerate().map(move |(c, ch)| {
                    let loc: Location = (r as isize, c as isize).into();
                    (loc, ch)
                })
            })
            .map(|(loc, ch)| {
                let spot = match ch {
                    '#' => GridSpace::Wall,
                    '.' => GridSpace::Space(vec![]),
                    '>' | 'v' | '^' | '<' => {
                        GridSpace::Space(vec![Blizzard::try_from(ch).ok().unwrap()])
                    }
                    _ => panic!("Unexpected char {}", ch),
                };
                (loc, spot)
            })
            .collect::<HashMap<Location, GridSpace>>();
        Grid {
            grid,
            width: content.lines().next().unwrap().len(),
            height: content.lines().count(),
        }
    }
}

struct BfsIterator<Node: Bfs> {
    queue: VecDeque<Node>,
}

impl<Node: Bfs> Iterator for BfsIterator<Node> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            None => None,
            Some(n) => {
                self.queue.extend(n.children());
                Some(n)
            }
        }
    }
}

struct FNode<Node> {
    node: Node,
    parent: Option<Rc<FNode<Node>>>,
}

impl<Node> FNode<Node> {
    fn to_list(self) -> Vec<Node> {
        let mut v: Vec<Node> = vec![];
        let mut fnode = self;
        loop {
            v.push(fnode.node);
            match fnode.parent {
                // TODO: get rid of this unwrap()
                Some(parent_fnode) => fnode = Rc::try_unwrap(parent_fnode).ok().unwrap(),
                None => break,
            }
        }
        v
    }
}

fn find_pop<T>(v: &mut Vec<T>, f: impl Fn(&T) -> bool) -> Option<T> {
    for (index, item) in v.iter().enumerate() {
        if f(item) {
            return Some(v.remove(index));
        }
    }
    return None;
}

/// Bfs is meant to be implemented by a node object. If a Node has a node.children() function that
/// is an iterator over children, then you can bfs() it
trait Bfs: Sized {
    // TODO: make children() return an iterator over Self instead
    fn children(&self) -> Vec<Self>;

    fn bfs(self) -> BfsIterator<Self> {
        // the first one is cloned, but none of the others
        BfsIterator {
            queue: VecDeque::from(vec![self]),
        }
    }

    fn find_path(self, f: impl Fn(&Self) -> bool) -> Option<Vec<Self>> {
        let mut queue: VecDeque<FNode<Self>> = vec![FNode {
            node: self,
            parent: None,
        }]
        .into();
        while let Some(fnode) = queue.pop_back() {
            let children = fnode.node.children();
            let handle = Some(Rc::new(fnode));
            let mut child_fnodes = children
                .into_iter()
                .map(|node| FNode {
                    node,
                    parent: handle.clone(),
                })
                .collect::<Vec<_>>();
            match find_pop(&mut child_fnodes, |fnode: &FNode<Self>| f(&fnode.node)) {
                None => queue.extend(child_fnodes),
                // TODO: want drop queue before this
                Some(fnode) => return Some(fnode.to_list()),
            }
        }
        None
    }
}

struct BoardState {
    grid: Grid,
    player: Location,
}

impl Bfs for BoardState {
    fn children(&self) -> Vec<BoardState> {
        let BoardState {
            grid: current_grid,
            player,
        } = self;
        let next_grid = current_grid.next();
        let steps = [
            *player,
            *player + Direction::Up,
            *player + Direction::Down,
            *player + Direction::Right,
            *player + Direction::Left,
        ];
        let child_nodes: Vec<BoardState> = steps
            .iter()
            .filter(|loc| match next_grid.grid.get(&loc) {
                None => false,
                Some(GridSpace::Wall) => false,
                Some(GridSpace::Space(v)) if v.len() == 0 => true,
                Some(GridSpace::Space(_)) => false,
            })
            .map(|&player| BoardState {
                player,
                grid: next_grid.clone(),
            })
            .collect();
        child_nodes
    }
}

impl Grid {
    fn next(&self) -> Self {
        let oldgrid = &self.grid;
        let mut newgrid: HashMap<Location, GridSpace> = HashMap::new();
        for (loc, gridspace) in oldgrid {
            match gridspace {
                GridSpace::Space(blizzards) => {
                    for blizzard in blizzards {
                        let mut newloc: Location = *loc + *blizzard;
                        let Location(ref mut r, ref mut c) = newloc;

                        if *r <= 0 {
                            *r = self.height as isize - 2;
                        } else if *r >= self.height as isize - 1 {
                            *r = 1;
                        }

                        if *c <= 0 {
                            *c = self.width as isize - 2;
                        } else if *c >= self.width as isize - 1 {
                            *c = 1;
                        }

                        match newgrid.get_mut(&newloc) {
                            None => {
                                newgrid.insert(newloc, GridSpace::Space(vec![*blizzard]));
                            }
                            Some(GridSpace::Space(ref mut v)) => {
                                v.push(*blizzard);
                            }
                            Some(GridSpace::Wall) => {}
                        }
                    }
                }
                GridSpace::Wall => {
                    newgrid.insert(*loc, GridSpace::Wall);
                }
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            grid: newgrid,
        }
    }

    fn pprint(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let item = self.grid.get(&(r as isize, c as isize).into());
                let to_display = match item {
                    Some(GridSpace::Wall) => '#',
                    Some(GridSpace::Space(vec)) if vec.is_empty() => '.',
                    Some(GridSpace::Space(vec)) if vec.len() == 1 => vec[0].char(),
                    Some(GridSpace::Space(vec)) if vec.len() == 2 => '2',
                    Some(GridSpace::Space(vec)) if vec.len() == 3 => '3',
                    Some(GridSpace::Space(vec)) if vec.len() == 4 => '4',
                    Some(GridSpace::Space(vec)) if vec.len() == 5 => '5',
                    Some(GridSpace::Space(vec)) if vec.len() == 6 => '6',
                    Some(GridSpace::Space(vec)) if vec.len() == 7 => '7',
                    Some(GridSpace::Space(vec)) if vec.len() == 8 => '8',
                    Some(GridSpace::Space(vec)) if vec.len() == 9 => '9',
                    Some(GridSpace::Space(_)) => 'M',
                    None => '.',
                };
                print!("{}", to_display);
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse grid
    //   blizzard { direction }
    //   triple array of blizzards
    // make grid.next() fn that gives the next grid
    // bfs for (Grid, loc) tuples

    return Ok(());

    let content = fs::read_to_string("src/d24/input")?;
    let mut grid = Grid::from(content.as_str());
    for _ in 1..=10 {
        grid.pprint();
        println!("-----------------------------------------");
        grid = grid.next();
    }

    Ok(())
}
