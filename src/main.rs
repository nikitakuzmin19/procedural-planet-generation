use rand::Rng;

fn main() {
    let width = 30;
    let height = 10;

    // matrix for terrain grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    // init randomizer
    let mut rng = rand::thread_rng();

    // randomly create terrain
    for _y in 0..height {
        let mut row: Vec<char> = Vec::new();

        for _x in 0..width {
            let n = rng.gen_range(0..3);

            let tile = match n {
                0 => '.', // grass
                1 => '~', // water
                _ => '^', // mountains
            };

            row.push(tile);
        }

        grid.push(row);
    }

    // print terrain grid
    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
}