use plotters::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

pub struct NonStatBandit {
    pub arms: usize,
    pub rates: Vec<f64>,
}

#[allow(dead_code, unused_must_use)]
impl NonStatBandit {
    pub fn play(&mut self, arm: usize) -> i32 {
        let rate: f64 = self.rates[arm];
        self.rates
            .iter()
            .map(|x| x + 0.1 * rand::thread_rng().gen::<f64>());

        let random_num: f64 = rand::thread_rng().gen();
        if random_num < rate {
            1
        } else {
            0
        }
    }
}

#[allow(non_snake_case)]
pub struct AlphaAgent {
    pub epsilon: f64,
    pub Qs: Vec<f64>,
    pub alpha: f64,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl AlphaAgent {
    pub fn update(&mut self, action: usize, reward: i32) {
        self.Qs[action] += (reward as f64 - self.Qs[action]) * self.alpha;
    }
    pub fn get_action(&self) -> usize {
        let random_num: f64 = rand::thread_rng().gen();
        if random_num < self.epsilon {
            rand::thread_rng().gen_range(0..self.Qs.len()) as usize
        } else {
            self.Qs
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index)
                .unwrap() as usize
        }
    }
}

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runs = 200;
    let steps = 1_000;
    let mut all_rates = vec![[0f64; 200]; 1_000];

    for run in 0..runs {
        let mut bandit = NonStatBandit {
            arms: 10,
            rates: vec![rand::thread_rng().gen(); 10],
        };
        let mut agent = AlphaAgent {
            epsilon: 0.1,
            Qs: vec![0f64; 10],
            alpha: 0.8,
        };
        let mut total_reward = 0;

        for step in 0..steps {
            let action = agent.get_action();
            let reward = bandit.play(action);
            agent.update(action, reward);
            total_reward += reward;
            all_rates[step][run] = total_reward as f64 / (step as f64 + 1f64);
        }
    }

    // draw a graph
    let mut avg_rates = vec![];
    let mut points_all: Vec<(f64, f64)> = vec![];
    for i in 0..steps {
        let mean_rates = all_rates[i].iter().sum::<f64>() / all_rates[i].len() as f64;
        avg_rates.push(mean_rates);
        points_all.push(((i + 1) as f64, mean_rates));
    }
    let (_, rates_max) = avg_rates
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));

    let root =
        BitMapBackend::new("output/non_stationary/rates.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Alpha const update", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, 0f64..rates_max)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(points_all, &RED))?;

    Ok(())
}
