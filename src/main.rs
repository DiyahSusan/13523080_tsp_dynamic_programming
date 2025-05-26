// use std::f64; 
// mod tsp;

// use tsp::{solve_tsp};

// fn main() {
//     // Contoh 1: Graf 4 node
//     let graph1: Vec<Vec<f64>> = vec![
//         vec![f64::INFINITY, 10.0, 15.0, 20.0],
//         vec![5.0, f64::INFINITY, 9.0, 10.0],
//         vec![6.0, 13.0, f64::INFINITY, 12.0],
//         vec![8.0, 8.0, 9.0, f64::INFINITY],
//     ];

//     println!("--- Contoh 1: Graf 4 Node ---");
//     let solution1 = solve_tsp(&graph1);
//     println!("Bobot Minimum: {}", solution1.min_cost);
//     println!("Jalur Optimal: {:?}", solution1.path); 

//     // Contoh 2: Graf 5 node
//     let graph2: Vec<Vec<f64>> = vec![
//         vec![f64::INFINITY, 2.0, 4.0, 1.0, 5.0],
//         vec![2.0, f64::INFINITY, 3.0, 2.0, 4.0],
//         vec![4.0, 3.0, f64::INFINITY, 1.0, 2.0],
//         vec![1.0, 2.0, 1.0, f64::INFINITY, 3.0],
//         vec![5.0, 4.0, 2.0, 3.0, f64::INFINITY],
//     ];

//     println!("\n--- Contoh 2: Graf 5 Node ---");
//     let solution2 = solve_tsp(&graph2);
//     println!("Bobot Minimum: {}", solution2.min_cost);
//     println!("Jalur Optimal: {:?}", solution2.path);

//     // Contoh 3: Graf 3 node (sederhana)
//     let graph3: Vec<Vec<f64>> = vec![
//         vec![f64::INFINITY, 10.0, 15.0],
//         vec![10.0, f64::INFINITY, 20.0],
//         vec![15.0, 20.0, f64::INFINITY],
//     ];

//     println!("\n--- Contoh 3: Graf 3 Node ---");
//     let solution3 = solve_tsp(&graph3);
//     println!("Bobot Minimum: {}", solution3.min_cost);
//     println!("Jalur Optimal: {:?}", solution3.path);
// }
// src/main.rs

use std::f64;
use std::io::{self, BufRead}; // Tambahkan ini untuk input/output
use std::str::FromStr; // Tambahkan ini untuk parsing string ke tipe data

// Modul TSP Anda
mod tsp;
use tsp::{solve_tsp};

// Fungsi baru untuk membaca graf dari stdin
fn read_graph_from_stdin() -> Vec<Vec<f64>> {
    let mut reader = io::stdin().lock();
    let mut line = String::new();

    println!("Masukkan jumlah node (N):");
    reader.read_line(&mut line).expect("Gagal membaca baris");
    let num_nodes: usize = line.trim().parse().expect("Input bukan angka");
    line.clear(); 

    let mut graph: Vec<Vec<f64>> = vec![vec![0.0; num_nodes]; num_nodes]; // inisialisasi dengan 0

    println!("Masukkan matriks bobot (N baris, N kolom). Gunakan 'inf' untuk infinity dan pisahkan angka dengan spasi:");
    println!("Contoh (untuk N=3):");
    println!("inf 10 15");
    println!("10 inf 20");
    println!("15 20 inf");

    for i in 0..num_nodes {
        println!("Masukkan baris ke-{} ", i + 1);
        line.clear();
        reader.read_line(&mut line).expect("Gagal membaca baris");

        let row_values: Vec<f64> = line
            .trim()
            .split_whitespace() // pisahkan string berdasarkan spasi
            .map(|s| {
                if s.eq_ignore_ascii_case("inf") { // cek jika input adalah "inf" (case-insensitive)
                    f64::INFINITY
                } else {
                    f64::from_str(s).expect("Tidak bisa parse bobot, pastikan angka atau 'inf'")
                }
            })
            .collect();

        if row_values.len() != num_nodes {
            panic!("Jumlah kolom di baris {} tidak sesuai dengan N ({})", i + 1, num_nodes);
        }

        graph[i] = row_values;
    }

    graph
}

fn main() {
    println!("--- Penyelesaian Traveling Salesman Problem ---");

    let graph = read_graph_from_stdin();

    if graph.is_empty() || graph[0].is_empty() {
        println!("Graf kosong atau tidak valid.");
        return;
    }

    // Pastikan graf adalah matriks persegi
    let num_nodes_check = graph.len();
    for row in &graph {
        if row.len() != num_nodes_check {
            println!("Graf bukan matriks persegi. Jumlah baris dan kolom harus sama.");
            return;
        }
    }
    
    // Asumsi: Fungsi solve_tsp Anda berada di src/tsp.rs dan dideklarasikan `pub`

    let solution = solve_tsp(&graph);

    println!("\n--- Hasil ---");
    if solution.min_cost == f64::INFINITY {
        println!("Tidak ada jalur yang mungkin (atau siklus) yang ditemukan.");
    } else {
        println!("Bobot Minimum: {}", solution.min_cost);
        println!("Jalur Optimal: {:?}", solution.path);
    }
}