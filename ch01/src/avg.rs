use rand::Rng;

#[allow(non_snake_case)]
fn main() {
    let seed: [u8; 32] = [13; 32];

    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    let mut Q = 0_f64;

    for n in 1..11 {
        let reward = rng.gen::<f64>();
        Q = Q + (reward - Q) / n as f64;
        // Q += (reward - Q) / n as f64;
        println!("{}", Q);
    }
}
