use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<GridSpace>,
    height: usize,
    width: usize,
}

impl<'a> From<&'a str> for Grid {
    fn from(content: &'a str) -> Self {
        let width = content.lines().next().unwrap().len();
        let height = content.lines().count();
        let mut grid: Vec<GridSpace> = vec![GridSpace::Wall; width * height];
        content
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars().enumerate().map(move |(c, ch)| {
                    // let loc: Location = (r as isize, c as isize).into();
                    ((r, c), ch)
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
            .for_each(|((r, c), gridspace)| {
                grid[r * width + c] = gridspace;
            });

        Grid {
            grid,
            width,
            height,
        }
    }
}

mod fp {
    use std::collections::VecDeque;
    use std::rc::Rc;

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

    /// function does not know to not revisit items it has already seen
    pub fn find_path<T>(
        start: T,
        f: impl Fn(&T) -> bool,
        get_children: impl Fn(&T) -> Vec<T>,
    ) -> Option<Vec<T>> {
        if f(&start) {
            return Some(vec![start]);
        }
        let mut queue: VecDeque<FNode<T>> = vec![FNode {
            node: start,
            parent: None,
        }]
        .into();
        while let Some(fnode) = queue.pop_front() {
            let children = get_children(&fnode.node);
            let handle = Some(Rc::new(fnode));
            let mut child_fnodes = children
                .into_iter()
                .map(|node| FNode {
                    node,
                    parent: handle.clone(),
                })
                .collect::<Vec<_>>();

            match find_pop(&mut child_fnodes, |fnode: &FNode<T>| f(&fnode.node)) {
                None => queue.extend(child_fnodes),
                Some(fnode) => {
                    // Drop all the things so Rc::unwrap() in to_list() works
                    drop(queue);
                    drop(child_fnodes);
                    drop(handle);
                    let mut l = fnode.to_list();
                    l.reverse();
                    return Some(l);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test_fp {
    use super::fp;

    #[test]
    fn test_basic() {
        let get_children: fn(&isize) -> Vec<isize> = |n| vec![2 * n, (2 * n) + 1];
        // first just make sure this doesn't fail for anything 1 - 100
        for target in 1..100 {
            fp::find_path(1_isize, |n| *n == target, get_children);
        }

        // then assert that these particular ones are the correct values
        let examples: Vec<(isize, _)> = vec![
            (14, Some(vec![1, 3, 7, 14])),
            (34, Some(vec![1, 2, 4, 8, 17, 34])),
            (59, Some(vec![1, 3, 7, 14, 29, 59])),
            (87, Some(vec![1, 2, 5, 10, 21, 43, 87])),
        ];
        for (target, expected) in examples.iter() {
            assert_eq!(fp::find_path(1, |n| n == target, get_children), *expected);
        }
    }

    #[test]
    fn test_empty() {
        let get_children: fn(&(isize, isize)) -> Vec<(isize, isize)> = |&(x, y)| {
            vec![(x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| 0 <= *x && *x <= 5 && 0 <= *y && *y <= 5)
                .collect::<Vec<_>>()
        };

        fp::find_path((0_isize, 0_isize), |&(x, y)| (x, y) == (2, 2), get_children);
        assert_eq!(
            fp::find_path(
                (0_isize, 0_isize),
                |&(x, y)| (x, y) == (10, 10),
                get_children,
            ),
            None
        );
    }
}

#[derive(Debug)]
struct BoardState {
    grid: Grid,
    player: Location,
}

impl BoardState {
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
            .filter(|loc| match next_grid.get(**loc) {
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
    fn get(&self, loc: Location) -> Option<&GridSpace> {
        let Location(r, c) = loc;
        if r < 0 || r >= self.height as isize || c < 0 || c > self.width as isize {
            return None;
        }
        let idx = self.loc_to_idx(loc);
        return Some(&self.grid[idx]);
    }
    fn loc_to_idx(&self, loc: Location) -> usize {
        return self.width * loc.0 as usize + loc.1 as usize;
    }
    fn idx_to_loc(&self, idx: usize) -> Location {
        return Location((idx / self.width) as isize, (idx % self.width) as isize);
    }
    fn next(&self) -> Self {
        let oldgrid = &self.grid;
        let Grid {
            grid: mut newgrid,
            width,
            ..
        } = self.clone_with_empty_spaces();
        // self.clone_with_empty_spaces().pprint();
        for (idx, gridspace) in oldgrid.iter().enumerate() {
            let loc = Location((idx / width) as isize, (idx % width) as isize);
            match gridspace {
                GridSpace::Space(blizzards) => {
                    for blizzard in blizzards {
                        let mut newloc: Location = loc + *blizzard;
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
                        let newidx = self.loc_to_idx(newloc);

                        match newgrid[newidx] {
                            GridSpace::Wall => panic!("logic error"),
                            GridSpace::Space(ref mut v) => v.push(*blizzard),
                        }
                    }
                }
                GridSpace::Wall => {} // walls should already exist from clone_with_empty_spaces
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            grid: newgrid,
        }
    }

    fn clone_with_empty_spaces(&self) -> Grid {
        let mut newgrid = self.grid.clone();
        for space in newgrid.iter_mut() {
            match space {
                GridSpace::Space(ref mut v) => v.clear(),
                _ => {}
            }
        }
        Grid {
            grid: newgrid,
            height: self.height,
            width: self.width,
        }
    }

    fn pprint(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let item = self.get((r as isize, c as isize).into());
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

    fn get_player_start_location(&self) -> Location {
        let start_locations = (0..self.width)
            .filter(|idx| self.get(self.idx_to_loc(*idx)) == Some(&GridSpace::Space(vec![])))
            .collect::<Vec<_>>();
        assert!(start_locations.len() == 1);
        return self.idx_to_loc(start_locations[0]);
    }

    fn get_player_end_location(&self) -> Location {
        let end_locations = ((self.width * (self.height - 1))..(self.width * self.height))
            .filter(|idx| self.get(self.idx_to_loc(*idx)) == Some(&GridSpace::Space(vec![])))
            .collect::<Vec<_>>();
        assert!(end_locations.len() == 1);
        return self.idx_to_loc(end_locations[0]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse grid
    //   blizzard { direction }
    //   triple array of blizzards
    // make grid.next() fn that gives the next grid
    // bfs for (Grid, loc) tuples

    let content = fs::read_to_string("src/d24/input")?;
    let grid = Grid::from(content.as_str());
    // for _ in 1..=10 {
    //     grid.pprint();
    //     println!("-----------------------------------------");
    //     grid = grid.next();
    // }

    let player = *&grid.get_player_start_location();
    let endloc = *&grid.get_player_end_location();
    let bs = BoardState { grid, player };
    let path = fp::find_path(bs, |bs| bs.player == endloc, |bs| bs.children());
    println!("num steps {:?}", path.map(|x| x.len()));

    Ok(())
}
