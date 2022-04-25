use rand::Rng;

pub struct Bandit {}

impl Bandit {
    pub fn init() -> Vec<f64> {
        let arm = 10;
        let mut rates: Vec<f64> = vec![];
        for _ in 0..arm {
            rates.push(rand::thread_rng().gen());
        }
        rates
    }
    pub fn play(&self, arm: usize) -> i32 {
        let rates: Vec<f64> = Bandit::init();
        let rate: f64 = rates[arm];
        let random_num: f64 = rand::thread_rng().gen();

        if random_num < rate {
            1
        } else {
            0
        }
    }
}

fn main() {
    let bandit = Bandit {};

    for _ in 0..3 {
        println!("{}", bandit.play(3));
    }
}
