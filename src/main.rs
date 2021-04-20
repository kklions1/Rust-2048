use rand::Rng;

struct Grid { 
    grid: [[i16; 4]; 4]
}

impl Grid { 
    fn new() -> Grid { 
        let init_grid = [
            [0, 0, 0, 0], 
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ];
        Grid { 
            grid: init_grid
        }
    }

    fn print_grid(&self) {
        let mut i = 0;
        let mut j: usize;
        while i < 4 {
            j = 0; 
            while j < 4 { 
                print!("{} ", self.grid[i][j]);
                j += 1; 
            }
            print!("\n");
            i += 1;
        }
    }

    fn generate_square(&mut self) {
        if !self.has_empty_space() { 
            return; 
        }

        let mut rng = rand::thread_rng();
        
        loop { 
            let rand_x: usize = rng.gen_range(0..4);
            let rand_y: usize = rng.gen_range(0..4);

            if self.grid[rand_x][rand_y] == 0 { 
                self.grid[rand_x][rand_y] = 2;
                break;
            }
        
        }
    } 
    
    fn has_empty_space(&self) -> bool { 
        for row in self.grid.iter() { 
            for col in row.iter() { 
                if *col == 0 as i16 { 
                    return true
                }
            }
        }
        false
    }

    fn has_equal_neighbor(&self) -> bool { 
        for x in 0..4 { 
            for y in 0..4 { 
                if y < 3 { 
                    if self.grid[x][y] == self.grid[x][y+1] { 
                        return true;
                    }
                }

                if x < 3 { 
                    if self.grid[x][y] == self.grid[x+1][y] { 
                        return true;
                    }
                }
            }
        }
        false
    }
}

enum Direction { 
    LEFT,
    RIGHT,
    UP,
    DOWN
}

fn main() {
    let mut game_board: Grid = Grid::new(); 

    loop { 
        game_board.print_grid();
        game_board.generate_square();
        if !game_board.has_empty_space() { 
            break;
        }
    }
    
}
