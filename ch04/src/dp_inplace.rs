use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut V = HashMap::new();
    V.insert("L1", 0f64);
    V.insert("L2", 0f64);

    let mut cnt = 0;
    loop {
        let mut t =
            0.5 * (-1 as f64 + 0.9 * V["L1"].clone()) + 0.5 * (1 as f64 + 0.9 * V["L2"].clone());
        let mut delta = (t.clone() - V["L1"].clone()).abs();
        *V.entry("L1").or_insert(0.0) = t;

        t = 0.5 * (0 as f64 + 0.9 * V["L1"].clone()) + 0.5 * (-1 as f64 + 0.9 * V["L2"].clone());
        delta = delta.max((t.clone() - V["L2"].clone()).abs());
        *V.entry("L2").or_insert(0.0) = t;

        cnt += 1;
        if delta < 0.0001 {
            println!("V {:?}", V);
            println!("cnt {:?}", cnt);
            break;
        }
    }

    Ok(())
}
