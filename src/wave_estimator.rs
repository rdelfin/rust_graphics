use std::collections::HashMap;

pub struct WaveEstimator {
    granularity: i32,
    alpha: f32,
    vals : HashMap<(i32, i32), f32>,
    speeds: HashMap<(i32, i32), f32>,
}

impl WaveEstimator {
    pub fn new(granularity: i32, alpha: f32, start_func: impl Fn(f32, f32) -> f32) -> WaveEstimator {
        let mut vals = HashMap::new();
        let mut speeds= HashMap::new();

        for x in -granularity..(granularity+1) {
            for y in -granularity..(granularity+1) {
                let x_f = x as f32 / granularity as f32;
                let y_f = y as f32 / granularity as f32;

                let val: f32;
                if x <= -granularity || x >= granularity || y <= -granularity || y >= granularity {
                    val = 0.0;
                } else {
                    val = start_func(x_f, y_f);
                }

                vals.insert((x, y), val);
                speeds.insert((x, y), 0.0);
            }
        }

        WaveEstimator { granularity, alpha, vals, speeds }
    }

    pub fn update(&mut self, dt: f32) {
        let mut new_vals: HashMap<(i32, i32), f32> = HashMap::new();
        let step  = 1.0 / self.granularity as f32;

        for x in -self.granularity..(self.granularity+1) {
            for y in -self.granularity..(self.granularity+1) {
                if x <= -self.granularity || x >= self.granularity || y <= -self.granularity || y >= self.granularity {
                    new_vals.insert((x, y), 0.0);
                } else {
                    let dfdx_right = (self.vals[&(x + 1, y)] - self.vals[&(x, y)]) / step;
                    let dfdx_left = (self.vals[&(x, y)] - self.vals[&(x-1, y)]) / step;
                    let d2fdx2 = (dfdx_right - dfdx_left) / step;

                    let dfdy_up = (self.vals[&(x, y+1)] - self.vals[&(x, y)]) / step;
                    let dfdy_down = (self.vals[&(x, y)] - self.vals[&(x, y-1)]) / step;
                    let d2fdy2 = (dfdy_up - dfdy_down) / step;

                    let d2fdt2 = self.alpha * (d2fdx2 + d2fdy2);

                    *self.speeds
                        .get_mut(&(x, y)).expect( "Speed does not contain all values") += d2fdt2 * dt;
                    new_vals.insert((x, y), self.vals[&(x, y)] + self.speeds[&(x, y)] * dt);
                }
            }
        }

        self.vals = new_vals;
    }

    pub fn get_val(&self, x: f32, y: f32) -> f32 {
        let x_rounded = (x * self.granularity as f32).round() as i32;
        let y_rounded = (y * self.granularity as f32).round() as i32;


        if x_rounded <= -self.granularity ||
            x_rounded >= self.granularity ||
            y_rounded <= -self.granularity ||
            y_rounded >= self.granularity {
            return 0.0;
        }

        return self.vals[&(x_rounded, y_rounded)];
    }
}
