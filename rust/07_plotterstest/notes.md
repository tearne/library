* Create fresh Ubuntu 20.04 Server
* Install rustup
* Reboot
* `cargo new plotterstest --bin`.
* Remotely open in VSCode using Remote Development and with rust-analyzer.
* `cargo run`
```linker "cc" not found``` 
* `sudo apt install build-essential`.  
* `cargo run` builds and runs hellp world.
* As per plotters [quick start](https://github.com/38/plotters#quick-start), add `plotters = "^0.3.0"` to dependencies, and then replace main with:
```
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
```
* `cargo run`
```
--- stderr
  thread 'main' panicked at '
  failed to execute command: No such file or directory (os error 2)
  is `cmake` not installed?

  build script failed, must exit now', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.45/src/lib.rs:894:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
* `sudo apt install cmake`
* `cargo run`
```
--- stderr
  configure: error: in `/home/user/plotterstest/target/debug/build/servo-fontconfig-sys-4e69794617a6cd74/out':
  configure: error: The pkg-config script could not be found or is too old.  Make sure it
  is in your PATH or set the PKG_CONFIG environment variable to the full
  path to pkg-config.

  Alternatively, you may set the environment variables FREETYPE_CFLAGS
  and FREETYPE_LIBS to avoid the need to call pkg-config.
  See the pkg-config man page for more details.

  To get pkg-config, see <http://pkg-config.freedesktop.org/>.
  See `config.log' for more details
  make: *** [makefile.cargo:83: /home/user/plotterstest/target/debug/build/servo-fontconfig-sys-4e69794617a6cd74/out/Makefile] Error 1
  thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
                           env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
                                                                                   "-f",
                                                                                   "makefile.cargo"]).status().unwrap().success()', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.9/build.rs:20:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
* `sudo apt install -y pkg-config`
* `cargo run`
```
 --- stderr
  configure: error: Package requirements (freetype2) were not met:

  No package 'freetype2' found

  Consider adjusting the PKG_CONFIG_PATH environment variable if you
  installed software in a non-standard prefix.

  Alternatively, you may set the environment variables FREETYPE_CFLAGS
  and FREETYPE_LIBS to avoid the need to call pkg-config.
  See the pkg-config man page for more details.
  make: *** [makefile.cargo:83: /home/user/plotterstest/target/debug/build/servo-fontconfig-sys-4e69794617a6cd74/out/Makefile] Error 1
  thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
                           env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
                                                                                   "-f",
                                                                                   "makefile.cargo"]).status().unwrap().success()', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.9/build.rs:20:5
```
* Random (searching suggests)[https://github.com/r-lib/systemfonts/issues/35] `sudo apt install libfontconfig1-dev`
* `cargo run`


