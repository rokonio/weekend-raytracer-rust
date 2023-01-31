pub fn filling_curve(xy: usize, width: usize, height: usize) -> (usize, usize) {
    let xy = (xy * 10) % (width * height + 1);
    let y = xy % height;
    let x = xy / height;
    (x, y)
}
