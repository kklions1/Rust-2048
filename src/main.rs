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

    fn generate_square(&self) { 
        if !self.has_empty_space() { 
            return; 
        }

        println!("{}", Rng.gen_range(0..10));

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
}

fn main() {
    let game_board: Grid = Grid::new(); 
    game_board.print_grid();
    assert_eq!(game_board.has_empty_space(), true);
    game_board.generate_square();
}
