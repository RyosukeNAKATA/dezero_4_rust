use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut V = HashMap::new();
    V.insert("L1", 0f64);
    V.insert("L2", 0f64);

    println!("V init {:?}", V);

    for i in 0..100 {
        let new_v = V.clone();

        let count_1 = V.entry("L1").or_insert(0.0);
        *count_1 = 0.5 * (-1 as f64 + 0.9 * new_v["L1"].clone())
            + 0.5 * (1 as f64 + 0.9 * new_v["L2"].clone());
        let count_2 = V.entry("L2").or_insert(0.0);
        *count_2 = 0.5 * (0 as f64 + 0.9 * new_v["L1"].clone())
            + 0.5 * (-1 as f64 + 0.9 * new_v["L2"].clone());

        println!("V {i} {:?}", new_v);
    }

    Ok(())
}
