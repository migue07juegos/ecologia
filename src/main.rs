use std::f32::consts::E;

use plotters::prelude::full_palette::GREY;
use plotters::prelude::*;
use plotters::series::AreaSeries;
use slint::SharedPixelBuffer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod wasm_backend;

slint::slint! {
    import { MainWindow } from "ui/appwindow.slint";
}

fn pdf(x: f64, y: f64, a: f64) -> f64 {
    const SDX: f64 = 0.1;
    const SDY: f64 = 0.1;
    let x = x as f64 / 10.0;
    let y = y as f64 / 10.0;
    a * (-x * x / 2.0 / SDX / SDX - y * y / 2.0 / SDY / SDY).exp()
}

fn render_plot(width: i32, height: i32) -> slint::Image {
    let mut pixel_buffer =
        SharedPixelBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    #[cfg(target_arch = "wasm32")]
    let backend = wasm_backend::BackendWithoutText { backend };

    let root = backend.into_drawing_area();

    let mut data2: Vec<f32> = Vec::new();
    let crecimiento = 0.04;
    let inicial = 200.0;
    let tiempo_max = 20;
    root.fill(&GREY).expect("error filling drawing area");

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            0..tiempo_max + 10,
            0..(100.0 + inicial * E.powf(crecimiento * tiempo_max as f32)) as i32,
        )
        .unwrap();
    chart.configure_mesh().draw().expect("error drawing");

    for i in 0..=tiempo_max {
        data2.push(inicial * E.powf(crecimiento * i as f32));
    }

    chart
        .draw_series(
            AreaSeries::new((0..).zip(data2.iter().map(|x| *x as i32)), 0, &RED.mix(0.2))
                .border_style(&RED),
        )
        .expect("error drawing series");

    root.present().expect("error presenting");
    drop(chart);
    drop(root);

    slint::Image::from_rgb8(pixel_buffer)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let main_window = MainWindow::new().unwrap();

    main_window.on_render_plot(render_plot);

    main_window.run().unwrap();
}
