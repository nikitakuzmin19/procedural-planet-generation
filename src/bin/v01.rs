// just random terrain generation

use rand::Rng;

fn main() {
    let width = 80;
    let height = 30;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..height {
        let mut row = Vec::new();

        for _ in 0..width {
            let n = rng.gen_range(0..3);

            let tile = match n {
                0 => '.',
                1 => '~',
                _ => '^',
            };
            row.push(tile);
        }
        grid.push(row);
    }

    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
}
