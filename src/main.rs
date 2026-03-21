//! v0.2 — Noise-based terrain: value noise height field + thresholds (~ . ^).

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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn fade(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn noise2d(x: f32, y: f32, seed: u32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - ix as f32;
    let fy = y - iy as f32;

    let v00 = hash2(ix, iy, seed);
    let v10 = hash2(ix + 1, iy, seed);
    let v01 = hash2(ix, iy + 1, seed);
    let v11 = hash2(ix + 1, iy + 1, seed);

    let u = fade(fx);
    let v = fade(fy);
    let a = lerp(v00, v10, u);
    let b = lerp(v01, v11, u);
    lerp(a, b, v)
}

fn main() {
    let width = 80;
    let height = 30;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut rng = rand::thread_rng();

    let freq = 0.1_f32;
    let offset_x: f32 = 1000.0;
    let offset_y: f32 = -2000.0;
    let seed: u32 = rng.gen();

    for _y in 0..height {
        let mut row = Vec::new();
        for _x in 0..width {
            let nx = (_x as f32) * freq + offset_x;
            let ny = (_y as f32) * freq + offset_y;
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

    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
}
