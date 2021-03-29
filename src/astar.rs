use grid::*;
use pathfinding::prelude::{absdiff, astar};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

pub fn find(t_grid: &Grid<i32>, start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
    impl Pos {
        fn distance(&self, other: &Pos) -> u32 {
            (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
        }

        fn successors(&self, grid: &Grid<i32>) -> Vec<(Pos, u32)> {
            let &Pos(x, y) = self;
            let mut t_suc_vec = vec![];
            for cur_pos in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
                if grid.get((cur_pos.0) as usize, (cur_pos.1) as usize) == Some(&0) {
                    t_suc_vec.push(Pos(cur_pos.0, cur_pos.1));
                }
            }
            t_suc_vec.into_iter().map(|p| (p, 1)).collect()
        }
    }
    let f_goal: Pos = Pos(goal.0, goal.1);
    let result = astar(
        &Pos(start.0, start.1),
        |p| p.successors(t_grid),
        |p| p.distance(&f_goal) / 3,
        |p| *p == f_goal,
    );
    //assert_eq!(result.expect("no path found").1, 4);
    let mut rt: Vec<(i32, i32)> = vec![];
    match result {
        Some(pos_vals) => {
            for cur_pos in pos_vals.0 {
                rt.push((cur_pos.0, cur_pos.1));
            }
        }
        None => {}
    }
    return rt;
}
