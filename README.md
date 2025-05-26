# Tugas IF2211 Strategi Algoritma
# Penyelesaian TSP dengan Dynamic Programming

## Contributors
<div align="center">

| **NIM**  | **Nama** |
| ------------- |:-------------:|
| 13523080   | Diyah Susan Nugrahani |

</div>

## Penjelasan Tugas
Tugas ini merupakan bentuk implementasi dari materi mata kuliah Strategi Algoritma terakit dynamic programming. Program yang dibuat merupakan solusi dari pencarian jalur optimal pada permasalahan TSP (Travelling Salesman Problem) menggunakan pendekatan dynamic programming. Pada solusi ini digunakan cara dynamic programming maju, dimana pencarian solusi dimulai dari informasi basis. Graf direpresentasikan sebagai matriks dua dimensi dimana index merepresentasikan node dan nilai merepresentasikan bobot jalur. 

## Bahasa yang Digunakan
Rust

## Cara Menjalankan Program 
1. Install Rust dan Cargo  
``` sh
https://www.rust-lang.org/tools/install
```
3. Clone repository  
4. Jalankan command  
``` sh
cargo run
```
5. Inputkan matriks dua dimensi yang merepresentasikan graf  
6. Untuk jarak graf ke graf itu sendiri direpresentasikan sebagai inf  
7. Output yang akan dihasilkan adalah berupa bobot minimal dan jalur optimal untuk persoalan TSP  
