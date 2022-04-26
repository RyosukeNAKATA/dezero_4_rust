use plotters::prelude::*;
use rand::Rng;

pub struct Bandit {}

#[allow(dead_code)]
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

#[allow(non_snake_case)]
pub struct Agent {
    pub epsilon: f64,
    pub Qs: Vec<f64>,
    pub ns: Vec<f64>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Agent {
    pub fn update(&mut self, action: usize, reward: i32) {
        self.ns[action] += 1_f64;
        self.Qs[action] += (reward as f64 - self.Qs[action]) / self.ns[action];
    }
    pub fn get_action(&self) -> usize {
        let random_num: f64 = rand::thread_rng().gen();
        if random_num < self.epsilon {
            rand::thread_rng().gen_range(0..self.Qs.len()) as usize
        } else {
            self.Qs.iter().fold(0.0 / 0.0, |m, v| v.max(m)) as usize
        }
    }
}

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let steps = 1_000;
    let epsilon = 0.1;

    let bandit = Bandit {};
    let mut agent = Agent {
        epsilon: epsilon,
        Qs: vec![0f64; 10],
        ns: vec![0f64; 10],
    };
    let mut total_reward = 0;
    let mut total_rewards: Vec<f64> = vec![];
    let mut rates: Vec<f64> = vec![];

    for step in 0..steps {
        let action = agent.get_action();
        let reward = bandit.play(action);
        agent.update(action, reward);
        total_reward += reward;

        total_rewards.push(total_reward as f64);
        rates.push(total_reward as f64 / (step as f64 + 1f64));
    }

    println!("Total reward: {:?}", total_reward);

    let (rewards_min, rewards_max) = total_rewards
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));
    let (rates_min, rates_max) = rates
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));

    let mut points_total_rewards = vec![];
    let mut points_rates = vec![];
    for (i, val) in total_rewards.iter().enumerate() {
        points_total_rewards.push(((i + 1) as f64, *val));
    }
    for (i, val) in rates.iter().enumerate() {
        points_rates.push(((i + 1) as f64, *val));
    }

    let root =
        BitMapBackend::new("output/bandit_total_reward.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10f64, 10f64, 10f64, 10f64);
    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Total Reward", ("sans-serif", 20).into_font())
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, rewards_min..rewards_max)?;
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .draw()?;
    chart.draw_series(LineSeries::new(points_total_rewards, &RED))?;

    let root = BitMapBackend::new("output/bandit_rates.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10f64, 10f64, 10f64, 10f64);
    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Total Reward", ("sans-serif", 20).into_font())
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, rates_min..rates_max)?;
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .draw()?;
    chart.draw_series(LineSeries::new(points_rates, &RED))?;

    Ok(())
}
