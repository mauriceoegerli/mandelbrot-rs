fn calculate_point(c1: f64, c2: f64, iterations: i32) -> i32 {
    let mut z_x: f64 = 0.0;
    let mut z_y: f64 = 0.0;

    let mut duplication_vec: Vec<[f64; 2]> = Vec::new();
    let mut has_duplication: bool = false;

    let mut duplication_stop_index: i32 = 0;

    for i in 0..iterations {
        if z_x.abs() >= 2.0 || z_y.abs() >= 2.0 {
            break;
        }

        let old_z_x: f64 = z_x;
        println!("x{} y{} c{}", z_x, z_y, c1);
        z_x = z_x * z_x - z_y * z_y + c1;
        z_y = 2.0 * old_z_x * z_y + c2;

        println!(
            "con {} len {}",
            duplication_vec.contains(&[z_x, z_y]),
            duplication_vec.len()
        );

        duplication_stop_index = i;
        if duplication_vec.len() > 0 && duplication_vec.contains(&[z_x, z_y]) {
            has_duplication = true;
            break;
        }
        duplication_vec.push([z_x, z_y]);
        duplication_stop_index += 1;
    }

    if has_duplication && z_x.abs() <= 2.0 && z_y.abs() <= 2.0 {
        return 0;
    }
    return duplication_stop_index;
}

fn calculate_mandelbrot() {
    const X: u32 = 100;
    const Y: u32 = 100;

    // for y in 0..Y {
    // for x in 0..X {
    print!("{}", calculate_point(-0.7093749999999999, 1.071875, 30));
    // }
    // }
}

fn main() {
    calculate_mandelbrot();
}
