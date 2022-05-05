use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut V = HashMap::new();
    V.insert("L1", 0f64);
    V.insert("L2", 0f64);

    let mut cnt = 0;
    loop {
        let new_v = V.clone();

        let count_1 = V.entry("L1").or_insert(0.0);
        *count_1 = 0.5 * (-1 as f64 + 0.9 * new_v["L1"].clone())
            + 0.5 * (1 as f64 + 0.9 * new_v["L2"].clone());

        let mut delta = (count_1.clone() - new_v["L1"].clone()).abs();

        let count_2 = V.entry("L2").or_insert(0.0);
        *count_2 = 0.5 * (0 as f64 + 0.9 * new_v["L1"].clone())
            + 0.5 * (-1 as f64 + 0.9 * new_v["L2"].clone());

        delta = delta.max((count_2.clone() - new_v["L2"].clone()).abs());

        cnt += 1;
        if delta < 0.0001 {
            println!("V {:?}", V);
            println!("cnt {:?}", cnt);
            break;
        }
    }

    Ok(())
}
