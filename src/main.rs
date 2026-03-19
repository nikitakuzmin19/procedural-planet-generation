use rand::Rng;

fn hash2(ix: i32, iy: i32, seed: u32) -> f32 {
    let mut h = seed;
    h ^= (ix as u32).wrapping_mul(0x27d4eb2d);
    h = h.wrapping_add((iy as u32).wrapping_mul(0x165667b1));
    h ^= h >> 15;
    h = h.wrapping_mul(0xd168aaad);
    h ^= h >> 15;

    (h as f32) / (u32::MAX as f32)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t;
}

pub fn fade (t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t);
}

pub fn noise2d(x: f32, y: f32, seed: u32) -> f32 {

    // find lattice cell
    let ix = floor(x) as i32;
    let iy = floor(y) as i32;

    // get the position inside the square
    // its like getting how far you are from the left border
    // and from the bottom
    // might be like 0.2 or 0.6, or 0.88 or sum like that
    let fx = x - (ix as f32);
    let fy = y - (iy as f32);

    // generate pseudo random weights for each corner
    // might also be called corner values
    // 0.3 - 0.7
    //  |     |
    // 0.5 - 0.2
    let v00 = hash2(ix, iy, seed);
    let v10 = hash2(ix+1, iy, seed);
    let v01 = hash2(ix, iy+1, seed);
    let v11 = hash2(ix+1, iy+1, seed);

    // u is horizontal fade
    // basically how far you are from the left border toward the right
    // v is vertical fade
    // same goes for vertical one
    u = fade(fx);
    v = fade(fy);

    a = lerp(v00, v10, u);
    b = lerp(v01, v11, u);
    h = lerp(a, b, v);
}

fn main() {
    let width = 80;
    let height = 30;

    // matrix for terrain grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    // init randomizer
    let mut rng = rand::thread_rng();

    // how fast height changes
    let freq = 0.1;

    let offsetx: f32 = 1000.0
    let offsety: f32 = -2000.0

    let seed: u32 = rng.gen(); // generate random seed

    // randomly create terrain
    for _y in 0..height {
        let mut row: Vec<char> = Vec::new();

        for _x in 0..width {
            let nx = (_x as f32) * freq + offsetx;
            let ny = (_y as f32) * freq + offsety;

            let h = noise2d(nx, ny, seed);

            let tile = if h < 0.4 {
                '~'
            } else if h < 0.75 {
                '.'
            } else {
                '^'
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