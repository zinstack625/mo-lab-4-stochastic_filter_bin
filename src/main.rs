use plotters::prelude::*;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

fn needed_func(x: f64) -> f64 {
    x.sin() + 0.5f64
}

fn get_function(
    func: &dyn Fn(f64) -> f64,
    value_range: std::ops::Range<f64>,
    count: usize,
) -> (Vec<f64>, Vec<(f64, f64)>) {
    let step = (value_range.end - value_range.start) / (count as f64);
    let mut func_vec = Vec::<f64>::with_capacity(count);
    let mut plot_vec = Vec::<(f64, f64)>::with_capacity(count);
    let mut i = value_range.start;
    while i < value_range.end {
        func_vec.push(func(i));
        plot_vec.push((i, func(i)));
        i += step;
    }
    (func_vec, plot_vec)
}

fn find_bounds(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    //                        omega delta   omega delta
    let (mut min, mut max) = ((None, None), (None, None));
    for i in points.iter() {
        if min.0.is_none() || *min.0.as_ref().unwrap() > i.0 {
            min.0 = Some(i.0);
        }
        if min.1.is_none() || *min.1.as_ref().unwrap() > i.1 {
            min.1 = Some(i.1);
        }
        if max.0.is_none() || *max.0.as_ref().unwrap() < i.0 {
            max.0 = Some(i.0);
        }
        if max.1.is_none() || *max.1.as_ref().unwrap() < i.1 {
            max.1 = Some(i.1);
        }
    }
    let min = (min.0.unwrap(), min.1.unwrap());
    let max = (max.0.unwrap(), max.1.unwrap());
    (
        (-0.1f64 * min.0.abs(), -0.1f64 * min.1.abs()),
        (max.0 + 0.1f64 * max.0.abs(), max.1 + 0.1f64 * max.1.abs()),
    )
}

fn main() {
    let (func, noizy_plot) = get_function(
        &|x| needed_func(x) + thread_rng().gen_range(-0.25f64..0.25f64),
        0f64..PI,
        100,
    );
    let (filtered_3, filtered_3_coeffs) = stochastic_filter::filter(&func, 3, (0f64, PI));
    let (filtered_5, filtered_5_coeffs) = stochastic_filter::filter(&func, 5, (0f64, PI));

    let mut noizeless_plot = Vec::<(f64, f64)>::with_capacity(func.len());
    let mut filtered_3_plot = Vec::<(f64, f64)>::with_capacity(func.len());
    let mut filtered_5_plot = Vec::<(f64, f64)>::with_capacity(func.len());
    let step = PI / (100 as f64);
    let mut i = 0f64;
    for j in 0..func.len() {
        noizeless_plot.push((i, needed_func(i)));
        filtered_3_plot.push((i, filtered_3[j]));
        filtered_5_plot.push((i, filtered_5[j]));
        i += step;
    }

    let task1 = BitMapBackend::new("./task1.png", (1000, 1000)).into_drawing_area();
    task1.fill(&WHITE);
    let mut chart = ChartBuilder::on(&task1)
        .margin(20u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f64..PI, 0.45f64..1.75f64)
        .unwrap();
    chart.configure_mesh().x_desc("x").y_desc("f(x)").draw();
    chart
        .draw_series(LineSeries::new(noizy_plot.clone(), &RED))
        .unwrap()
        .label("Noizy signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(noizeless_plot.clone(), &GREEN))
        .unwrap()
        .label("Original signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart
        .draw_series(LineSeries::new(filtered_3_plot, &MAGENTA))
        .unwrap()
        .label("Filtered signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw();

    let task1_coeffs = BitMapBackend::new("./task1_coeffs.png", (500, 500)).into_drawing_area();
    task1_coeffs.fill(&WHITE);
    let (min, max) = find_bounds(&filtered_3_coeffs);
    let mut chart = ChartBuilder::on(&task1_coeffs)
        .margin(20u32)
        .x_label_area_size(30u32)
        .y_label_area_size(60u32)
        .build_cartesian_2d(min.0..max.0, min.1..max.1)
        .unwrap();
    chart.configure_mesh().x_desc("ω").y_desc("Δ").draw();
    chart.draw_series(
        filtered_3_coeffs
            .iter()
            .map(|coord| Circle::new(coord.clone(), 3u32, &BLUE)),
    );
    chart.draw_series(
        [(0f64, 0f64)]
            .iter()
            .map(|coord| Circle::new(coord.clone(), 3u32, &RED)),
    );

    let task2 = BitMapBackend::new("./task2.png", (1000, 1000)).into_drawing_area();
    task2.fill(&WHITE);
    let mut chart = ChartBuilder::on(&task2)
        .margin(20u32)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f64..PI, 0.45f64..1.75f64)
        .unwrap();
    chart.configure_mesh().x_desc("x").y_desc("f(x)").draw();
    chart
        .draw_series(LineSeries::new(noizy_plot, &RED))
        .unwrap()
        .label("Noizy signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(noizeless_plot, &GREEN))
        .unwrap()
        .label("Original signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart
        .draw_series(LineSeries::new(filtered_5_plot, &MAGENTA))
        .unwrap()
        .label("Filtered signal")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw();

    let task2_coeffs = BitMapBackend::new("./task2_coeffs.png", (500, 500)).into_drawing_area();
    task2_coeffs.fill(&WHITE);
    let (min, max) = find_bounds(&filtered_5_coeffs);
    let mut chart = ChartBuilder::on(&task2_coeffs)
        .margin(20u32)
        .x_label_area_size(30u32)
        .y_label_area_size(60u32)
        .build_cartesian_2d(min.0..max.0, min.1..max.1)
        .unwrap();
    chart.configure_mesh().x_desc("ω").y_desc("Δ").draw();
    chart.draw_series(
        filtered_5_coeffs
            .iter()
            .map(|coord| Circle::new(coord.clone(), 3u32, &BLUE)),
    );
    chart.draw_series(
        [(0f64, 0f64)]
            .iter()
            .map(|coord| Circle::new(coord.clone(), 3u32, &RED)),
    );
}
