struct Solution;

// impl Solution {
//     pub fn check_overlap(
//         radius: i32,
//         x_center: i32,
//         y_center: i32,
//         x1: i32,
//         y1: i32,
//         x2: i32,
//         y2: i32,
//     ) -> bool {
//         let closest_x = x_center.clamp(x1, x2);
//         let closest_y = y_center.clamp(y1, y2);
//
//         let dx = closest_x - x_center;
//         let dy = closest_y - y_center;
//
//         dx * dx + dy * dy <= radius * radius
//     }
// }

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        fn get_y(radius: i32, x_center: i32, y_center: i32, x: i32) -> i32 {
            let dx = x - x_center;
            let value = radius * radius - dx * dx;

            if value < 0 {
                return y_center;
            }

            (value as f64).sqrt() as i32 + y_center
        }

        for x in (x_center - radius)..=(x_center + radius) {
            let y = get_y(radius, x_center, y_center, x);

            if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
                return true;
            }
        }

        for x in (x_center - radius)..=(x_center + radius) {
            let dx = x - x_center;
            let value = radius * radius - dx * dx;

            if value < 0 {
                continue;
            }

            let y = y_center - (value as f64).sqrt() as i32;

            if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
                return true;
            }
        }

        false
    }
}

fn main() {
    let radius = 1;
    let x_center = 5;
    let y_center = 5;
    let x1 = -1;
    let y1 = -0;
    let x2 = 0;
    let y2 = 1;
    println!(
        "{}",
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2)
    );
}
