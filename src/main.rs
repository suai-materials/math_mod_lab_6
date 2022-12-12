extern crate core;

use std::io::{stdin};
use rand::Rng;


fn main() {
    println!("Введите размер матрицы в формате nxm(10x12)");
    let mut input_line = String::new();
    let mut rng = rand::thread_rng();
    stdin().read_line(&mut input_line).expect("Пожалуйста напишите что-то");
    let mut rev = input_line.split("x").map(|str| {match str.trim().parse::<usize>() {
        Ok(n) => {if n < 2 || n > 10{panic!("m и n должны быть в пределах от 2 до 10")} else {n}}
        Err(_) => {panic!("m и n должны быть целыми числами")}
    }});
    let m = match rev.next() {
        None => {2}
        Some(n) => {n}
    };
    let n = match rev.next(){
        None => {2}
        Some(n) => {n}
    };
    let mut matrix = vec![vec![0; n]; m] ;

    for i in 0..m{
        for j in 0..n{
            matrix[i][j] = rng.gen_range(2..=12);
            print!("{}\t", matrix[i][j])
        }
        println!()
    }
    let mini_max: i32 = match matrix.iter().map(|col| {col.iter().min()}).max().expect("WTF") {
        None => {0}
        Some(n) => {*n}
    };
    let mut max_of_rows = vec![0; m];
    for i in 0..m{
        max_of_rows[i] = match matrix[i].iter().max() {
            None => {0}
            Some(n) => {*n}
        };
    }
    let maxi_min = match max_of_rows.iter().min() {
        None => {0}
        Some(n) => {*n}
    };
    println!("Нижняя точка: {maxi_min}, Верхняя точка {mini_max}");
    if maxi_min == mini_max{
        println!("Чистые стратегии есть")
    } else {
        println!("Чистых стратегий не существует")
    }

}
