use std::vec;

/**
    1 - player
    2 - goal
    3 - obstacles
    4 - path

    g-cost is cost from node (grid point) to player
    h-cost is cost from node (grid point) to goal
    f-cost = g+h
 */

fn main() {
    let mut my_grid: Grid = Grid {
        x: 2,
        y: 2,
        grid: vec![vec![]]
    };

    my_grid = my_grid.set(10, 5);
    my_grid.position(0, 0, 5, 4);

    my_grid.print();
    my_grid.a_star(&[0, 0], &[5, 4], &mut [0, 0]);
    my_grid.print();
}


struct Grid {
    x: i64,
    y: i64,
    grid: Vec<Vec<i64>>
}

impl Grid {
    fn print(&self){
        for row in self.grid.iter() {
            for &val in row {
                print!("{}", val);
            }
            println!()
        }
    }

    fn set(&mut self, x:i64, y: i64) -> Grid{
        let new_grid: Vec<Vec<i64>> = vec![vec![0; y.try_into().unwrap()]; x.try_into().unwrap()];

        Grid{
            x: x,
            y: y,
            grid: new_grid
        }
    }

    /* add points */
    fn update(&mut self, x: usize, y: usize, _type: i64) {
        self.grid[x][y] = _type;
    }

    /* Seeker @ x1, y1 and end point at x2, y2 */
    fn position(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        self.grid[x1][y1] = 1;
        self.grid[x2][y2] = 2;

        // obstacle
        self.grid[0][2] = 3;
        self.grid[1][2] = 3;
        self.grid[2][2] = 3;
    }

    fn a_star(&mut self, player: &[i64], goal: &[i64], node: &mut [i64]){
        let mut found: bool = false;
        let mut closed_set: Vec<(i64, i64)> = Vec::new();

        while !found {
            let mut f_arr: Vec<(f32, Vec<i64>)> = Vec::with_capacity(8);
            let mut path: Vec<(i64, i64)> = Vec::with_capacity(100);
            path.push(
                (node[0], node[1])
            );

            let directions = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1), /*(0, 0),*/ (0, 1),
                (1, -1), (1, 0), (1, 1)
            ]; /* Eight for the eight directions */

            for (d_x, d_y) in directions {
                let new_x = node[0] as isize + d_x;
                let new_y = node[1] as isize + d_y;

                if new_x == goal[0] as isize && new_y == goal[1] as isize {
                    found = true;
                    break;
                }

                if new_x >= 0 && new_x < self.x as isize && new_y >= 0 && new_y < self.y as isize && self.grid[new_x as usize][new_y as usize] != 3 {
                    let g: f32 = (((player[0] - new_x as i64) as f32).powi(2) + ((player[1] - new_y as i64) as f32).powi(2)).sqrt();
                    let h: f32 = (((goal[0] - new_x as i64) as f32).powi(2) + ((goal[1] - new_y as i64) as f32).powi(2)).sqrt();
                    let f: f32 = g + h;

                    if !closed_set.contains(&(new_x as i64, new_y as i64)) {
                        f_arr.push((f, vec![new_x as i64, new_y as i64]));
                    }
                }
            }

            if !f_arr.is_empty() {
                let mut min = &f_arr[0].0;
                for (i, _p) in f_arr.iter() {
                    if i < min {
                        min = i;
                    }
                }

                for (i, p) in f_arr.iter() {
                    if i == min {
                        node[0] = p[0];
                        node[1] = p[1];

                        closed_set.push(
                            (node[0], node[1])
                        );

                        self.update(p[0] as usize, p[1] as usize, 4);
                    }
                }
            } else {
                println!("No path found.");
                break;
            }

            println!("{:?}", path);
        }

        if found {
            println!("Path to goal found."); 
        }
    }
}
