use rustracer::units::color;
fn main() {
    let image_width = 1024;
    let image_height = 1024;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let color = color::Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.25,
            );
            color::write_color(color);
        }
    }
    eprintln!("Done");
}
