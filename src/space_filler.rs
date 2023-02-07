// use rand::seq::SliceRandom;
// use rand::thread_rng;

use crate::*;

static CURVE: Lazy<Vec<(usize, usize)>> = Lazy::new(|| {
    let mut curve = Vec::new();
    for xy in 0..(WIDTH * HEIGHT) {
        curve.push({
            let xy = (xy * 10) % (WIDTH * HEIGHT + 1);
            let y = xy % HEIGHT;
            let x = xy / HEIGHT;
            (x, y)
        });
    }
    // curve.shuffle(&mut thread_rng());
    curve
});

pub fn filling_curve(xy: usize) -> (usize, usize) {
    *CURVE.get(xy).unwrap()
}
