
// http://www.plasmaphysics.org.uk/programs/coll2d_cpp.htm
pub fn collision_2d(
    m1: f64,
    m2: f64,
    elastic: f64,
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
    (mut vx1, mut vy1): (f64, f64),
    (mut vx2, mut vy2): (f64, f64)
) -> ((f64, f64), (f64, f64)) {
    let m21 = m2 / m1;
    let mut x21 = x2 - x1;
    let y21 = y2 - y1;
    let vx21 = vx2 - vx1;
    let vy21 = vy2 - vy1;

    let vx_cm = (m1 * vx1 + m2 * vx2) / (m1 + m2);
    let vy_cm = (m1 * vy1 + m2 * vy2) / (m1 + m2);


    //     *** return old velocities if balls are not approaching ***
    if (vx21 * x21 + vy21 * y21) >= 0.0 {
        return ((vx1, vy1), (vx2, vy2));
    }


    //     *** I have inserted the following statements to avoid a zero divide;
    //         (for single precision calculations,
    //          1.0E-12 should be replaced by a larger value). **************

    let fy21 = 1.0E-12 * y21.abs();
    if x21.abs() < fy21 {
        let sign: f64;
        if x21 < 0.0 {
            sign = -1.0;
        } else {
            sign = 1.0;
        }
        x21 = fy21 * sign;
    }

    //     ***  update velocities ***
    let a = y21 / x21;
    let dvx2 = -2.0 * (vx21 + a * vy21) / ((1.0 + a * a) * (1.0 + m21));
    vx2 = vx2 + dvx2;
    vy2 = vy2 + a * dvx2;
    vx1 = vx1 - m21 * dvx2;
    vy1 = vy1 - a * m21 * dvx2;

    //     ***  velocity correction for inelastic collisions ***
    vx1 = (vx1 - vx_cm) * elastic + vx_cm;
    vy1 = (vy1 - vy_cm) * elastic + vy_cm;
    vx2 = (vx2 - vx_cm) * elastic + vx_cm;
    vy2 = (vy2 - vy_cm) * elastic + vy_cm;

    ((vx1, vy1), (vx2, vy2))
}
