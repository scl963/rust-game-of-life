use rand::prelude::*;

fn main() {
    // Create mutable grid as vector of vectors
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 40]; 40];
    let game_length = 10;

    // Randomly turn on some cells with ~33% probability
    for row in grid.iter_mut() {
        for x in 0..row.len() {
            let random_num: f64 = rng.gen();
            let state = random_num < 0.33;
            row[x] = if state == true { 1 } else { 0 };
        }
    }

    println!("Original Grid");
    for row in &grid {
        println!("{:?}", row);
    }

    for generation in 0..game_length {
        let mut new_grid: Vec<Vec<u8>> = vec![];

        for row_index in 0..grid.len() {
            let mut new_row: Vec<u8> = vec![];
            let row = &grid[row_index];
            for column_index in 0..row.len() {
                let cell = row[column_index];
                let mut live_neighbors = 0;

                if row_index > 0 {
                    if grid[row_index - 1][column_index] == 1 {
                        live_neighbors += 1;
                    }

                    if column_index > 0 {
                        if grid[row_index - 1][column_index - 1] == 1 {
                            live_neighbors += 1;
                        }
                    }

                    if column_index + 1 < row.len() {
                        if grid[row_index - 1][column_index + 1] == 1 {
                            live_neighbors += 1;
                        }
                    }
                }

                if row_index + 1 < grid.len() {
                    if grid[row_index + 1][column_index] == 1 {
                        live_neighbors += 1;
                    }

                    if column_index > 0 {
                        if grid[row_index + 1][column_index - 1] == 1 {
                            live_neighbors += 1;
                        }
                    }

                    if column_index + 1 < row.len() {
                        if grid[row_index + 1][column_index + 1] == 1 {
                            live_neighbors += 1;
                        }
                    }
                }

                if column_index + 1 < row.len() {
                    if grid[row_index][column_index + 1] == 1 {
                        live_neighbors += 1;
                    }
                }

                if column_index > 0 {
                    if grid[row_index][column_index - 1] == 1 {
                        live_neighbors += 1;
                    }
                }

                if cell == 1 && live_neighbors > 1 && live_neighbors < 4 {
                    new_row.push(1);
                } else if cell == 0 && live_neighbors == 3 {
                    new_row.push(1);
                } else {
                    new_row.push(0);
                }

                // println!("Cell {} {} has {} live neighbors.", row_index, column_index, live_neighbors);
            }

            new_grid.push(new_row);
        }

        grid = new_grid;

        println!("Grid for generation {}", generation);
        for row in &grid {
            println!("{:?}", row);
        }
    }
}
