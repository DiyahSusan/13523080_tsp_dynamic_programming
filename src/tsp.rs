// use std::collections::HashMap;
use std::f64;

pub struct TSPSolution{
    pub min_cost: f64,
    pub path: Vec<usize>,
}

pub fn solve_tsp(graph: &Vec<Vec<f64>>) -> TSPSolution{
    let num_nodes = graph.len();
    let mut dp: Vec<Vec<f64>> = vec![vec![f64::INFINITY; num_nodes]; 1 << num_nodes];
    let mut parent: Vec<Vec<Option<usize>>> = vec![vec![None; num_nodes]; 1 << num_nodes];

    dp[1 << 0][0] = 0.0; // starting at node 0
    
    // iterasi smeua kemungkinan himpunan kota ynag dikunjungi
    for mask in 1..(1 << num_nodes){
        for last_nodes in 0..num_nodes{
            if (mask & (1 << last_nodes) != 0 ) && dp[mask][last_nodes] != f64::INFINITY{
                for current_node in 0..num_nodes{
                    if (mask & (1<< current_node) == 0) && graph[last_nodes][current_node] != f64::INFINITY{
                        let new_mask = mask | (1 << current_node);
                        let cost_to_current = dp[mask][last_nodes] + graph[last_nodes][current_node];
                        if cost_to_current < dp[new_mask][current_node] {
                            dp[new_mask][current_node] = cost_to_current;
                            parent[new_mask][current_node] = Some(last_nodes); // simpan node sebelumnya
                        }
                    }
                }
            }
        }
    }
    
    // kembali ke node awal dari setiap node terakhir
    let mut min_cost = f64::INFINITY;
    let mut last_node_path: Option<usize> = None;
    
    let final_mask = (1 << num_nodes)-1;
    for node in 0..num_nodes{
        if dp[final_mask][node] != f64::INFINITY && graph[node][0] != f64::INFINITY {
            let cost = dp[final_mask][node] + graph[node][0];
            if cost < min_cost {
                min_cost = cost;
                last_node_path = Some(node);
            }
        }
    }
    
    // rekonstruksi jalur
    let mut path: Vec<usize> = Vec::new();
    if min_cost != f64::INFINITY && last_node_path.is_some(){
        let mut current_mask = final_mask;
        let mut current_node = last_node_path.unwrap();
    
        // menelusuri kembali jalur
        while let Some(prev_node) = parent[current_mask][current_node]{
            path.insert(0, current_node + 1); // tambahkan ke jalur
            current_mask ^= 1 << current_node; // hapus node dari mask
            current_node = prev_node; // pindah ke node sebelumnya
        }
    
        path.insert(0, 1); // tambahkan node awal ke jalur
        path.push(1); // tambahkan node awal ke akhir jalur untuk kembali ke awal
    
    }
    TSPSolution {
            min_cost,
            path,
    }
}



