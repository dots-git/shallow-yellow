impl f64 {
    fn approx(b: f64, tolerance: f64) -> bool {
        b < self + tolerance && b > self - tolerance
    }
}

fn circular_exponential(&value: f64, &change_rate: f64, target: f64, acceleration: f64, acceleration_modifier: f64, drag: f64, delta_time: f64) {
    let mut moving_twd = value - change_rate / f64::log(drag);

    // Cap the change_rate if slowing would lead to exceeding the target
    if moving_twd > target + 0.01 {
        change_rate = (value - target) * f64::log(drag);
        moving_twd = value - change_rate / f64::log(drag);
    }

    if moving_twd < target - 0.01 {
        if f64::atan(change_rate) + acceleration * delta_time < f64::consts::PI {
            change_rate = acceleration_modifier * f64::tan(f64::atan(change_rate / acceleration_modifier) - acceleration * delta_time)
        }
        else {
            change_rate += change_rate - acceleration_modifier * f64::tan(f64::atan(change_rate / acceleration_modifier) - acceleration * delta_time)
        }
        moving_twd = value - change_rate / f64::log(drag)
    }

    if moving_twd.approx(target, 0.01) {
        change_rate = f64::powf(drag, delta_time);
        change_rate = ((value + change_rate * delta_time) * f64::log(drag))
    }
}