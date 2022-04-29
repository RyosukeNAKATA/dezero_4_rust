use ch01::bandit::{Agent, Bandit};

use plotters::prelude::*;
use rand::Rng;

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runs = 200;
    let steps = 1_000;
    let epsilon = 0.1;
    let mut all_rates = vec![[0f64; 200]; 1_000];

    for run in 0..runs {
        let mut bandit = Bandit {
            arms: 10,
            rates: vec![rand::thread_rng().gen(); 10],
        };
        let mut agent = Agent {
            epsilon: epsilon,
            Qs: vec![0f64; 10],
            ns: vec![0f64; 10],
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
    // draw a graph
    let root = BitMapBackend::new("output/bandit_avg/rates_epsilon_0.1.png", (1280, 960))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Total Reward", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..1_000f64, 0f64..rates_max)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(points_all, &RED))?;

    Ok(())
}
