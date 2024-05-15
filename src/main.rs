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

    root.fill(&GREY).expect("error filling drawing area");

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    chart.configure_mesh().draw().expect("error drawing");
    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    chart
        .draw_series(
            AreaSeries::new((0..).zip(data.iter().map(|x| *x)), 0, &RED.mix(0.2))
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
