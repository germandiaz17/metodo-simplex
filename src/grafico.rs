use plotters::prelude::*;

pub fn graficar(titulo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("solucion.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(titulo, ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..8f64, 0f64..8f64)?;

    chart.configure_mesh()
        .x_desc("x (sillas)")
        .y_desc("y (mesas)")
        .draw()?;

    // Restricción 1: 2x + 3y = 12 → y = (12 - 2x) / 3
    chart.draw_series(LineSeries::new(
        (0..=60).map(|x| x as f64 / 10.0)
            .map(|x| (x, (12.0 - 2.0 * x) / 3.0))
            .filter(|&(_, y)| y >= 0.0),
        &BLUE,
    ))?.label("2x + 3y ≤ 12 (carpintería)")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Restricción 2: x + 2y = 8 → y = (8 - x) / 2
    chart.draw_series(LineSeries::new(
        (0..=80).map(|x| x as f64 / 10.0)
            .map(|x| (x, (8.0 - x) / 2.0))
            .filter(|&(_, y)| y >= 0.0),
        &RED,
    ))?.label("x + 2y ≤ 8 (pintura)")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Región factible (vértices)
    let vertices = vec![
        (0.0, 0.0),
        (6.0, 0.0),
        (0.0, 4.0),
    ];

    chart.draw_series(
        std::iter::once(Polygon::new(vertices.clone(), &GREEN.mix(0.3)))
    )?;

    // Puntos vértices
    chart.draw_series(vertices.iter().map(|&(x, y)| {
        Circle::new((x, y), 5, BLACK.filled())
    }))?;

    // Punto óptimo
    chart.draw_series(std::iter::once(
        Circle::new((6.0, 0.0), 8, RED.filled())
    ))?.label("Óptimo (6, 0) Z=240")
      .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    // Función objetivo Z = 240: y = (240 - 40x) / 50
    chart.draw_series(LineSeries::new(
        (0..=60).map(|x| x as f64 / 10.0)
            .map(|x| (x, (240.0 - 40.0 * x) / 50.0))
            .filter(|&(x, y)| x >= 0.0 && y >= 0.0),
        &RGBColor(128, 0, 128),
    ))?.label("Z = 240 (óptimo)")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(128, 0, 128)));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Gráfico guardado en solucion.png");
    Ok(())
}
