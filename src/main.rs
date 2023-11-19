const A: f64 = 0.6;

const WINDOW_WIDT: usize = 120;
const WINDOW_HEIGHT: usize = 50;
const LIGHT: &str = ".,-~:;=!*#$@";

fn show(v: &[Vec<char>]) {
    print!("\x1b[2J");
    print!("\x1b[H");
    for row in v.iter() {
        println!(" ");
        for c in row.iter() {
            print!("{}", c);
        }
    }
}

fn torus() -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = vec![vec![' '; WINDOW_WIDT]; WINDOW_HEIGHT];
    let mut th: f64 = 0.0;
    let threthold = 2.0 * (std::f32::consts::PI as f64);
    println!("threthold: {}", threthold);
    while th < threthold {
        let mut fi: f64 = 0.0;
        while fi < threthold {
            let d = -4.0 / (6.0 + f64::sin(fi));
            let t = 2.0 + f64::cos(fi);
            let x = 30.0 * (A * t * f64::cos(th) * d) + 40.0;
            let y = 15.0 * (A * t * f64::sin(th) * d) + 25.0;
            let p = -f64::sin(fi);
            if x > 0.0 && y > 0.0 {
                v[y as usize][x as usize] = if p > 0.0 {
                    LIGHT.chars().collect::<Vec<char>>()[(6.0 * p + 5.0) as usize]
                } else {
                    ' '
                };
            }

            fi += 0.07;
        }
        th += 0.02;
    }
    v
}

fn main() {
    let v = torus();
    show(&v);
    //     for cs in v.iter() {
    //         println!("{}", cs.iter().collect::<String>())
    //     }
}
