use ncurses::*;
use rand::Rng;

mod args;
use args::ConwaysArgs;
use clap::Parser;

trait Life {
    fn tick(&self) -> Self;

    fn print(&self);

    fn draw(&self, dead: char, live: char);

    fn noise(&self) -> Self;
}

impl Life for Vec<Vec<bool>> {
    fn tick(&self) -> Self {
        let mut new: Vec<Vec<bool>> = self.clone();

        let x_end = self[0].len()-1;
        let y_end = self.len()-1;

        for (y, row) in self.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {


                let (nx, px) = 
                    match x {
                        0 => (x_end, 1),
                        x if x == x_end => (x-1, 0),
                        x => (x-1, x+1)
                };

                let (ny, py) = 
                    match y {
                        0 => (y_end, 1),
                        y if y == y_end => (y-1, 0),
                        y => (y-1, y+1)
                };

                let adj = [
                    self[ny][nx],
                    self[y][nx],
                    self[py][nx],

                    self[ny][x],
                    self[py][x],

                    self[ny][px],
                    self[y][px],
                    self[py][px],
                ];

                let count = adj.into_iter().filter(|cell| *cell).count();

                new[y][x] = match (cell, count) {
                    (_, 3) | (true, 2) => true,
                    _ => false,
                };
                
            }
        }

        new
    }

    fn print(&self) {
        for row in self {
            for item in row {
                if *item {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn draw(&self, dead: char, live: char) {

        for (y, row) in self.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                mvaddch(x as i32, y as i32, if *cell { live as u32 } else { dead as u32 }  );
            }
        }

    }

    fn noise(&self) -> Self {
        let mut rng = rand::thread_rng();
        self.iter()
            .map(|row| 
                row.iter()
                    .map(|_| rng.gen())
                    .collect()
                    )
            .collect()
    }
}

fn main() {
    let args = ConwaysArgs::parse();

    initscr();
    noecho();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    let mut arr = vec![vec![false;rows as usize];cols as usize];
    arr = arr.noise();

    loop {
        clear();
        arr.draw(args.dead, args.live);
        arr = arr.tick();
        napms(args.tick_rate) ;
        refresh();
    }
}
