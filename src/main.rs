use std::{f32::consts, f64::consts::PI, thread, time::Duration};

const WINDOW_WIDT: usize = 80;
const WINDOW_HEIGHT: usize = 22;
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

fn torus(rx: f64) -> Vec<Vec<char>> {
    let cx = f64::cos(rx);
    let sx = f64::sin(rx);
    let mut v: Vec<Vec<char>> = vec![vec![' '; WINDOW_WIDT]; WINDOW_HEIGHT];
    let mut front: Vec<Vec<f64>> = vec![vec![0.0; WINDOW_WIDT]; WINDOW_HEIGHT];
    let mut th: f64 = 0.0;
    let threthold = 2.0 * (std::f32::consts::PI as f64);
    let mut min = 10.0;
    let mut max = -1.0;
    while th < threthold {
        let mut fi: f64 = 0.0;
        while fi < threthold {
            let cfi = f64::cos(fi);
            let sfi = f64::sin(fi);
            let cth = f64::cos(th);
            let sth = f64::sin(th);
            let t = 2.0 + cth;
            let z = cx * sth;
            let d = 1.0 / (5.0 + z);
            let e = cx * sfi * t - sx * sfi * t;
            let x = 30.0 * d * t * cfi + 40.0;
            let y = 15.0 * d * e + 12.0;
            let p = cx * sfi * cth - sx * sfi * cth - sx * sfi * cth - cx * sth;
            // println!("x: {}, y: {}", x, y);
            if p > max {
                max = p;
            }
            if p < min {
                min = p;
            }
            if 80.0 > x && x > 0.0 && 22.0 > y && y > 0.0 && d > front[y as usize][x as usize] {
                v[y as usize][x as usize] = if p > 0.0 {
                    LIGHT.chars().collect::<Vec<char>>()[if p < 12.0 / 8.0 {
                        (8.0 * p) as usize
                    } else {
                        11
                    }]
                } else {
                    ' '
                };
                front[y as usize][x as usize] = d;
            }

            fi += 0.07;
        }
        th += 0.02;
    }
    // println!("min: {}, max: {}", min, max);
    v
}

fn main() {
    let mut rx = 0.0;
    loop {
        if rx > std::f32::consts::PI as f64 {
            rx = 0.0;
        };
        rx += 0.04;
        show(&torus(rx));
        thread::sleep(Duration::from_micros(30000))
    }
    //let v = torus(rx);
    //for cs in v.iter() {
    //    println!("{}", cs.iter().collect::<String>())
    //}
}
