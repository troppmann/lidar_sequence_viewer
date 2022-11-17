// from Game Programming Gems 4 chapter 1.10 and unitys smooth damp
pub fn smooth_damp(
    current: f32,
    target: f32,
    current_velocity: &mut f32,
    smooth_time: f32,
    delta_time: f32,
) -> f32 {
    let omega = 2.0 / smooth_time;
    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - target;
    let temp = (*current_velocity + omega * change) * delta_time;
    *current_velocity = (*current_velocity - omega * temp) * exp;
    let mut output = target + (change + temp) * exp;
    if (target - current > 0.0) == (output > target) {
        output = target;
        *current_velocity = 0.0;
    }

    return output;
}

pub fn lerp(alpha: f32, min: f32, max: f32) -> f32 {
    (1.0 - alpha) * min + alpha * max
}

pub fn inv_lerp(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}
