// v2.2 — height + moisture noise, biome colors

use image::{Rgb, RgbImage};
use rand::Rng;

// turns a grid point (ix, iy) into a stable [0, 1] value
fn hash2(ix: i32, iy: i32, seed: u32) -> f32 {
    let mut h = seed;
    h ^= (ix as u32).wrapping_mul(0x27d4eb2d);
    h = h.wrapping_add((iy as u32).wrapping_mul(0x165667b1));
    h ^= h >> 15;
    h = h.wrapping_mul(0xd168aaad);
    h ^= h >> 15;
    (h as f32) / (u32::MAX as f32) // map to 0..1
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// smoothstep — without this the grid lines show through
fn fade(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn noise2d(x: f32, y: f32, seed: u32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - ix as f32; // fractional position inside the cell
    let fy = y - iy as f32;

    // sample the four corners of the cell
    let v00 = hash2(ix, iy, seed);
    let v10 = hash2(ix + 1, iy, seed);
    let v01 = hash2(ix, iy + 1, seed);
    let v11 = hash2(ix + 1, iy + 1, seed);

    let u = fade(fx);
    let v = fade(fy);
    let a = lerp(v00, v10, u); // interpolate top edge
    let b = lerp(v01, v11, u); // interpolate bottom edge
    lerp(a, b, v) // interpolate between them
}

fn sample_height(wx: f32, wy: f32, seed: u32) -> f32 {
    noise2d(wx, wy, seed)
}

// lower frequency so moisture zones are broader than the terrain features
fn sample_moisture(wx: f32, wy: f32, seed: u32) -> f32 {
    let m_seed = seed.wrapping_add(0x9E3779B9); // golden ratio offset — keeps moisture visually unrelated to height
    noise2d(wx * 0.55, wy * 0.55, m_seed)
}

fn biome_color(height: f32, moisture: f32) -> Rgb<u8> {
    // thresholds tuned by eye
    const DEEP_OCEAN: f32 = 0.38;
    const SHALLOW_OCEAN: f32 = 0.44;
    const BEACH: f32 = 0.475;
    const HIGHLAND: f32 = 0.62;
    const PEAKS: f32 = 0.78;

    if height < DEEP_OCEAN {
        Rgb([20, 60, 130])
    } else if height < SHALLOW_OCEAN {
        Rgb([54, 116, 181])
    } else if height < BEACH {
        Rgb([210, 200, 150])
    } else if height < HIGHLAND {
        // same elevation, moisture picks the biome
        if moisture < 0.32 {
            Rgb([194, 178, 128]) // desert
        } else if moisture < 0.58 {
            Rgb([95, 164, 72]) // grassland
        } else {
            Rgb([45, 100, 40]) // forest
        }
    } else if height < PEAKS {
        if moisture > 0.55 {
            Rgb([80, 115, 70]) // wet highland
        } else {
            Rgb([120, 118, 110]) // rock
        }
    } else {
        Rgb([220, 220, 230]) // snow
    }
}

fn main() {
    let width = 200;
    let height = 200;

    let mut rng = rand::thread_rng();

    let freq = 0.05_f32; // lower = more zoomed in, larger terrain features
    let offset_x: f32 = 1000.0;
    let offset_y: f32 = -2000.0; // offset so we're not sampling near the origin
    let seed: u32 = rng.r#gen();

    let mut img = RgbImage::new(width, height);

    for py in 0..height {
        for px in 0..width {
            let wx = (px as f32) * freq + offset_x; // pixel to world space
            let wy = (py as f32) * freq + offset_y;

            let h = sample_height(wx, wy, seed);
            let m = sample_moisture(wx, wy, seed);
            img.put_pixel(px, py, biome_color(h, m));
        }
    }

    img.save("terrain.png").expect("failed to save terrain.png");
}
