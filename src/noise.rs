#![allow(unused)]

use crate::*;

pub static NOISE: OnceCell<[(f32, f32); SAMPLE_PER_PIXEL]> = OnceCell::new();

#[inline]
pub fn randx(i: usize) -> f32 {
    NOISE.get().unwrap().get(i).unwrap().0
    // rand::random()
}

#[inline]
pub fn randy(i: usize) -> f32 {
    NOISE.get().unwrap().get(i).unwrap().1
    // rand::random()
}

pub fn blue_noise() -> Vec<glm::Vec2> {
    let mut r = StdRng::seed_from_u64(SEED);
    let mut samples = Vec::new();
    for sample_index in 0..SAMPLE_PER_PIXEL {
        let mut best_score = f32::NEG_INFINITY;
        let mut best_candidate: Option<glm::Vec2> = None;
        for _ in 0..sample_index + 1 {
            let mut score = f32::INFINITY;
            let candidate = glm::Vec2::new(r.gen(), r.gen());
            for test_sample in &samples {
                let test_score = toroidal_distance(&candidate, test_sample);
                score = score.min(test_score);
            }
            if score > best_score {
                best_score = score;
                best_candidate = Some(candidate);
            }
        }
        samples.push(best_candidate.unwrap_or(glm::Vec2::new(0.0, 0.5)));
    }
    samples
}

pub fn toroidal_distance(a: &glm::Vec2, b: &glm::Vec2) -> f32 {
    let mut dx = (b.x - a.x).abs();
    let mut dy = (b.y - a.y).abs();
    if dx > 0.5 {
        dx = 1.0 - dx;
    }

    if dy > 0.5 {
        dy = 1.0 - dy;
    }
    (dx * dx + dy * dy).sqrt()
}
pub fn set_noise() {
    let noise = blue_noise()
        .iter()
        .map(|p| (p.x, p.y))
        .collect::<Vec<(f32, f32)>>();
    NOISE
        .set(noise.try_into().unwrap())
        .expect("NOISE set twice. This is a bug");
}
