use grid::*;

pub fn run(finder: fn(&Grid<i32>, (i32, i32), (i32, i32)) -> ()) {
    let world_grid = grid![
        [0,0,1]
        [0,1,0]
        [0,1,0]
        [0,1,0]
        [0,0,0]
    ];
    finder(&world_grid, (0, 0), (2, 2));
}
