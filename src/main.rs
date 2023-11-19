use std::{thread, time::Duration};
const A: f64 = 0.3;

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

fn torus(rx: f64, rz: f64) -> Vec<Vec<char>> {
    let cx = f64::cos(rx);
    let sx = f64::sin(rx);
    let cz = f64::cos(rz);
    let sz = f64::sin(rz);
    let mut v: Vec<Vec<char>> = vec![vec![' '; WINDOW_WIDT]; WINDOW_HEIGHT];
    let mut th: f64 = 0.0;
    let threthold = 2.0 * (std::f32::consts::PI as f64);
    while th < threthold {
        let mut fi: f64 = 0.0;
        while fi < threthold {
            let cfi = f64::cos(fi);
            let sfi = f64::sin(fi);
            let cth = f64::cos(th);
            let sth = f64::sin(th);
            let t = 2.0 + cfi;
            let d = -4.0 / (6.0 + t * sx * sth + cx * sfi);
            let x = 30.0 * d * A * (t * (cz * cth - sz * sx * sth) + sz * sx * sfi) + 40.0;
            let y = 15.0 * d * (A * (t * (sz * cth - cz * cx * sth) - cz * sx * sfi)) + 36.0;
            let p = A * A * t * (sx * sx - cz * cz) * sth * (cx * sth * sfi - sx * cfi);
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
    let mut rx = 0.0;
    let mut rz = 0.0;
    loop {
        show(&torus(rx, rz));
        rx += 0.00004;
        rz += 0.00002;
        thread::sleep(Duration::from_micros(30000))
    }
    //     let v = torus(rx, rz);
    //     for cs in v.iter() {
    //         println!("{}", cs.iter().collect::<String>())
    //     }
}
