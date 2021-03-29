use grid::*;
use std::fs;
use std::str;

pub fn solve(num: i32, finder: fn(&Grid<i32>, (i32, i32), (i32, i32)) -> Vec<(i32, i32)>) {
    let cur_str = format!("robot_recovery/robotrecovery{}.txt", num);
    let buffer_data = fs::read(&cur_str).expect("Unable to read file");
    //println!("{:#?}", data);
    let data = str::from_utf8(&buffer_data).unwrap();
    let split_data = data.split("\n");
    let vec_data = split_data.collect::<Vec<&str>>();
    let mut cur_row = 0;
    let mut dim_y = 0;
    let mut num_robots = 0;
    let mut grid_vec: Vec<i32> = vec![];
    let mut robots: Vec<(i32, i32)> = vec![];
    let mut dest = (0, 0);
    for x in vec_data {
        if cur_row == 0 {
            let first_split = x.split(" ").collect::<Vec<&str>>();
            dim_y = first_split[1].parse().unwrap();
            num_robots = first_split[2].parse().unwrap();
        } else {
            let mut cur_col = 0;
            for t_char in x.chars() {
                match t_char {
                    'X' => {
                        grid_vec.push(1);
                    }
                    '.' => {
                        grid_vec.push(0);
                    }
                    'R' => {
                        robots.push((cur_row - 1, cur_col));
                        grid_vec.push(0);
                    }
                    'E' => {
                        dest = (cur_row - 1, cur_col);
                        grid_vec.push(0);
                    }
                    _ => {}
                }
                cur_col += 1;
            }
        }
        cur_row += 1;
    }
    let world_grid = Grid::from_vec(grid_vec, dim_y);
    println!("{:?} {:?}", robots[0], dest);
    finder(&world_grid, robots[0], dest);
    //println!("{:#?}", (dim_x, dim_y, num_robots));
}
