use grid::*;
use itertools::Itertools;
use std::fs;
use std::str;

pub fn solve(num: i32, finder: fn(&Grid<i32>, (i32, i32), (i32, i32)) -> Vec<(i32, i32)>) {
    let cur_str = format!("robot_recovery/robotrecovery{}.txt", num);
    let buffer_data = fs::read(&cur_str).expect("Unable to read file");
    //println!("{:#?}", data);
    let data = str::from_utf8(&buffer_data).unwrap();
    let vec_data = data.split("\n").collect::<Vec<&str>>();
    let mut cur_row = 0;
    let mut dim_x = 0;
    let mut dim_y = 0;
    let mut num_robots = 0;
    let mut grid_vec: Vec<i32> = vec![];
    let mut robots: Vec<(i32, i32)> = vec![];
    let mut dest = (0, 0);
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in vec_data {
        if cur_row == 0 {
            let first_split = x.split(" ").collect::<Vec<&str>>();
            dim_x = first_split[0].parse::<i32>().unwrap();
            dim_y = first_split[1].parse::<i32>().unwrap();
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
    drop(cur_str);
    drop(data);
    let world_grid = Grid::from_vec(grid_vec, dim_y as usize);

    let mut solution = String::from("");
    while !robots.is_empty() {
        //break; //remove this.
        let mut current = vec![];
        for cur_robot in &robots {
            current.push(find_move_set(
                finder(&world_grid, *cur_robot, dest),
                &directions,
            ));
        }
        current.sort_by(|a, b| b.1.len().cmp(&(a.1.len() as usize)));

        for i in 0..(current.len() - 1) {
            let mut min_len: i32 = -1;
            let mut min_path: Vec<&(i32, i32)> = vec![&(-1, -1)];
            let mut min_path_str: String = String::from("");
            for j in (i + 1)..(current.len()) {
                let t_path_all =
                    find_move_set(finder(&world_grid, robots[i], robots[j]), &directions);
                let t_path = t_path_all.1;
                let t_path_str = t_path_all.0;
                let t_path_len = t_path.len() as i32;
                // t_path robot[i] to robot[j] -->moveset
                if (min_len == -1 || t_path_len < min_len)
                    && t_path_len <= current[j].1.len() as i32
                {
                    min_len = t_path_len;
                    min_path = t_path;
                    min_path_str.push_str(&t_path_str);
                }
                // if (min_len == -1 || t_path len < min_len) && min_len < path from j to destination.{
                //     update minlen and minpath
                // }
            }
            if min_len != -1 {
                println!("{}", min_len);
                break;
                // robot1tup, robot2tup
                // while(robot1tup != robot2tup){
                //     find path and moveset
                //     apply to robot1up and robot2up
                //     add moveset path to min_path and add moveset str to moves
                // }
                // apply tup to all robots.
                // deduplicate and remove robots done.
                // add moves to solution
                // break
            }
            if i == current.len() - 1 {
                //code for doing astar for the closest one.
                //should this be the longest one?
            }
        }
    }
    println!("{}", solution);
    // let cur_move_set = find_move_set(finder(&world_grid, robots[0], dest));
    // println!("{:?}", robots);
    // move_robots(cur_move_set.1, &mut robots, dim_x, dim_y, &world_grid);
    // robots = deduplicate(robots);
    // robots = remove_robots_done(robots, dest);
    // println!("{:?}", robots);

    /*
    let mut solution = String::from("");
    while robots isn't empty. {
        create a vector current (robot clone tup, vec)
        add path -->moveset from position to destination (for all positions) and add to vector as (robot, vec)
        sort vector by path length, largest to smallest


        for i in current length {
            int min_len --> minimum length of a path to the next one. initially -1
            vec tup min_path --> the path ^
            str moves = path string
            for j in i+1..currentlength{
                t_path robot[i] to robot[j] -->moveset
                if (min_len == -1 || t_path len < min_len) && min_len < path from j to destination.{
                    update minlen and minpath
                }
            }
            if min_len != -1 {
                robot1tup, robot2tup
                while(robot1tup != robot2tup){
                    find path and moveset
                    apply to robot1up and robot2up
                    add moveset path to min_path and add moveset str to moves
                }
                apply tup to all robots.
                deduplicate and remove robots done.
                add moves to solution
                break
            }
        }

    }

         */

    // while !robots.is_empty() {

    // }

    // println!("{:?} {:?}", robots[0], dest);
    // let rt = String::from("");
    //while robots.len() != 0 {}
    //println!("{}", find_move_set(finder(&world_grid, robots[0], dest)));

    // println!("{}", rt);
    //println!("{:#?}", (dim_x, dim_y, num_robots));
}

fn find_move_set(moves: Vec<(i32, i32)>, dir: &Vec<(i32, i32)>) -> (String, Vec<&(i32, i32)>) {
    let mut prev = moves[0];
    let mut rt_moves = String::from("");
    let mut rt_moves_tuple: Vec<&(i32, i32)> = vec![];
    for pos in moves {
        let cur_tup = (pos.0 - prev.0, pos.1 - prev.1);
        match cur_tup {
            (1, 0) => {
                rt_moves.push('D');
                rt_moves_tuple.push(&dir[0]);
            }
            (-1, 0) => {
                rt_moves.push('U');
                rt_moves_tuple.push(&dir[1]);
            }
            (0, 1) => {
                rt_moves.push('R');
                rt_moves_tuple.push(&dir[2]);
            }
            (0, -1) => {
                rt_moves.push('L');
                rt_moves_tuple.push(&dir[3]);
            }
            _ => {}
        }
        prev = pos;
    }
    return (rt_moves, rt_moves_tuple);
}

fn move_robots(
    move_set: Vec<(i32, i32)>,
    rbts: &mut Vec<(i32, i32)>,
    d_x: i32,
    d_y: i32,
    cur_grid: &Grid<i32>,
) {
    for cur_move in move_set {
        for cur_robot in &mut *rbts {
            let t_x = cur_move.0 + cur_robot.0;
            let t_y = cur_move.1 + cur_robot.1;
            if t_x < 0 || t_x >= d_x || t_y < 0 || t_y >= d_y {
                continue;
            } else {
                if cur_grid.get(t_x as usize, t_y as usize) == Some(&0) {
                    cur_robot.0 = t_x;
                    cur_robot.1 = t_y;
                }
            }
        }
    }
}

fn deduplicate(robot_vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    robot_vec.into_iter().unique().collect()
}

fn remove_robots_done(robot_vec: Vec<(i32, i32)>, destination: (i32, i32)) -> Vec<(i32, i32)> {
    robot_vec
        .into_iter()
        .filter(|cur_robot_pos| {
            !(cur_robot_pos.0 == destination.0 && cur_robot_pos.1 == destination.1)
        })
        .collect()
}
