fn main() {
    // Constant radius 2

    // Iterator to count number of loops
    let mut i: u64 = 0;

    // Number of sides, let's start with a hexagon
    let mut n: f64 = 6.0;

    // Length of one side, starts at 2.0 for a hexagon if the radius is 2
    let mut s: f64 = 2.0;

    // Starting value for current pi estimate
    let mut pi: f64 = 3.0;

    // Infinite loop to calculate pi
    loop {
        i += 1;
        // Double the number of sides
        n = n * 2.0;

        // big_opposite is half the length of one side of the hexagon
        let big_opposite: f64 = s / 2.0;

        let big_adjacent: f64 = (4.0 - big_opposite.powi(2)).sqrt();

        let small_opposite: f64 = 2.0 - big_adjacent;

        // This is the length of the new side
        s = (big_opposite.powi(2) + small_opposite.powi(2)).sqrt();

        if (n * s)/4.0 == pi {
            println!("Pi Found! with {} loops, with a polygon of {} sides", i, n);
            break;
        }
        // Since we know our radius is 2.0 our diameter is 4.0
        // The circumference is the number of sides times the length of each
        // side
        pi = (n * s)/4.0;

        println!("{}", pi);
    }

}
