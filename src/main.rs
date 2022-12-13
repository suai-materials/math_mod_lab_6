extern crate core;

use std::io::{stdin};
use rand::Rng;


fn main() {
    println!("Введите размер матрицы в формате nxm(10x12)");
    let mut input_line = String::new();
    let mut rng = rand::thread_rng();
    stdin().read_line(&mut input_line).expect("Пожалуйста напишите что-то");
    let mut rev = input_line.split("x").map(|str| {
        match str.trim().parse::<usize>() {
            Ok(n) => { if n < 2 || n > 10 { panic!("m и n должны быть в пределах от 2 до 10") } else { n } }
            Err(_) => { panic!("m и n должны быть целыми числами") }
        }
    });
    let m = match rev.next() {
        None => { 2 }
        Some(n) => { n }
    };
    let n = match rev.next() {
        None => { 2 }
        Some(n) => { n }
    };
    let mut matrix = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(2..=12);
            print!("{}\t", matrix[i][j])
        }
        println!()
    }
    let mini_max: i32 = match matrix.iter().map(|row| { row.iter().min() }).max().expect("WTF") {
        None => { 0 }
        Some(n) => { *n }
    };
    let mut max_of_cols = vec![0; m];
    for j in 0..n {
        max_of_cols[j] = match matrix.iter().map(|row| { row[j] }).rev().max() {
            None => { 0 }
            Some(n) => { n }
        };
    }
    let maxi_min = match max_of_cols.iter().min() {
        None => { 0 }
        Some(n) => { *n }
    };
    println!("Нижняя цена игры: {maxi_min}, Верхняя цена игры {mini_max}");
    if maxi_min == mini_max {
        println!("Чистые стратегии есть");
        let mut strategy_count = 0;
        for (i, row) in matrix.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                if *el == mini_max {
                    println!("Стратегия № {}", strategy_count + 1);
                    println!("Первый игрок ходит {} вариантом", i + 1);
                    strategy_count += 1;
                }
                if *el == mini_max {
                    println!("Второй игрок ходит {} вариантом", j + 1);
                }
            }
        }
    } else {
        println!("Чистых стратегий не существует");
        if n == 2 && m == 2 {
            // Формулы взяты из интернета
            let x1 = f64::from(matrix[1][1] - matrix[1][0]) / f64::from(matrix[0][0] + matrix[1][1] - matrix[1][0] - matrix[0][1]);
            let x2 = f64::from(matrix[0][0] - matrix[0][1]) / f64::from(matrix[0][0] + matrix[1][1] - matrix[1][0] - matrix[0][1]);
            let y1 = f64::from(matrix[1][1] - matrix[0][1]) / f64::from(matrix[0][0] + matrix[1][1] - matrix[1][0] - matrix[0][1]);
            let y2 = f64::from(matrix[0][0] - matrix[1][0]) / f64::from(matrix[0][0] + matrix[1][1] - matrix[1][0] - matrix[0][1]);
            println!("x1 = {x1} x2 = {x2} y1 = {y1} y2 = {y2}");
            let v = f64::from(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) / f64::from(matrix[0][0] + matrix[1][1] - matrix[1][0] - matrix[0][1]);
            println!("Цена игры {v}")
        }
    }
}
