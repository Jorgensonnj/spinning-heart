use std::{ str, thread::sleep, time::Duration, sync::{ Arc, atomic::{ AtomicBool, Ordering } } };
use signal_hook::{ flag, consts::TERM_SIGNALS };

fn main() -> Result<(), std::io::Error>{

    // Create thread-safe pointer
    let term_now = Arc::new(AtomicBool::new(false));
    // Resgister common process SIGs with pointer
    for sig in TERM_SIGNALS {
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    // Clear screen and hide cursor
    print!("\x1b[2J\x1b[?25l");

    let heart_size = 0.3;
    let heart_beat_size = 0.03;

    let mut tick : f64 = 0.0;
    // loop until interupt
    while !term_now.load(Ordering::Relaxed) {
        let mut z_axis_buffer = [0.0; 100 * 40];
        let mut maximum_z_axis = 0.0;
        let (cosine, sine) = (tick.cos(), tick.sin());

        for y_axis in -50..50 {
            let y_axis_fraction = f64::from(y_axis) / 100.0;

            // Add beating effect
            let beat = heart_size + heart_beat_size * (0.5 + 0.5 * (tick * 6.0 + y_axis_fraction * 2.0).sin()).powi(8);

            for x_axis in -50..50 {
                let x_axis_fraction = f64::from(x_axis) / 100.0;

                let y_position = 1.2 * y_axis_fraction - x_axis_fraction.abs() * 2.0 / 3.0;
                let x_position = -x_axis_fraction * x_axis_fraction - y_position.powi(2) + beat * beat;

                // Heart formula
                let z_value = x_position.sqrt() / (2.0 - y_axis_fraction);

                if z_value < 0.0 {
                  continue;
                }

                let mut z_tick = -z_value;
                while z_tick < z_value {
                    // Rotate
                    let x_rotate = x_axis_fraction * cosine - z_tick * sine;
                    let z_rotate = x_axis_fraction * sine   + z_tick * cosine;

                    // Add perspective 
                    let p = 1.0 + z_rotate / 2.0;

                    // Convert to screen coordinates
                    let x_window = (( x_rotate * p + 0.5) * 70.0 + 10.0).round();
                    let y_window = ((-y_axis_fraction * p + 0.5) * 38.0 + 2.0).round();

                    let idx = (x_window + 100.0 * y_window) as usize;
                    if z_axis_buffer[idx] <= z_rotate {
                        z_axis_buffer[idx] = z_rotate;
                        if z_rotate > maximum_z_axis {
                            maximum_z_axis = z_rotate;
                        }
                    }
                    z_tick += z_value / 6.0;
                }
            }
        }

        let mut window_line = [0u8; 100 * 40];
        for (index, z_axis) in z_axis_buffer.iter().enumerate() {
          window_line[index] = if index % 100 == 0 {
              10
            } else {
              let str = " .,-~:;=!*#$&@".as_bytes();
              str[(z_axis / maximum_z_axis * 13.0).round() as usize]
            };
        };
        print!("\x1b[H{}", str::from_utf8(&window_line).unwrap());

        tick += 0.004;
        sleep(Duration::from_millis(3));
    }

    // show cursor again
    print!("\n\x1b[?25h Tick: {}\n", tick);

    Ok(())
}
