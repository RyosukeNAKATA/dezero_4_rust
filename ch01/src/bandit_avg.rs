use ch01::bandit;

use plotters::prelude::*;

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runs = 200;
    let steps = 1_000;
    let epsilon = 0.01;
    let mut all_rates = vec![[0f64; 200]; 1_000];

    for run in 0..runs {
        let bandit = bandit::Bandit {};
        let mut agent = bandit::Agent {
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
    let mut avg_rates = vec![];
    let mut points_all: Vec<(f64, f64)> = vec![];
    for i in 0..steps {
        let mean_rates = all_rates[i].iter().sum::<f64>() / all_rates[i].len() as f64;
        avg_rates.push(mean_rates);
        points_all.push(((i + 1) as f64, mean_rates));
    }
    let (rates_min, rates_max) = avg_rates
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));

    let root =
        BitMapBackend::new("output/bandit_avg_rates_0.01.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10f64, 10f64, 10f64, 10f64);
    let mut chart = ChartBuilder::on(&root)
        .caption("Bandit Total Reward", ("sans-serif", 20).into_font())
        .x_label_area_size(50f64)
        .y_label_area_size(50f64)
        .build_cartesian_2d(0f64..1_000f64, rates_min..rates_max)?;
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .draw()?;
    chart.draw_series(LineSeries::new(points_all, &RED))?;

    Ok(())
}
