mod backlight;
use std::env;

fn usage(bad_arg: Option<&str>) -> ! {
    let mut retval = 0;
    let argzero = env::args()
        .nth(0)
        .unwrap_or_else(|| "pbacklight".to_owned());

    if let Some(arg) = bad_arg {
        retval = 1;
        eprintln!("{}: unrecognized argument '{}'", argzero, arg);
    }

    eprintln!(
        "usage: {} [options]
  where options are:
  -help
  -version
  -set <percentage>
  -inc <percentage>
  -dec <percentage>
  -get",
        argzero
    );
    std::process::exit(retval)
}

fn version() -> ! {
    eprintln!("pbacklight 0.1.0");
    std::process::exit(0)
}

enum PrgmMode {
    Get,
    Set,
    Add,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut to_percent: i16 = 0;

    let mode = match args.len() {
        1 => PrgmMode::Get,
        2 => match args[1].as_str() {
            "-version" => {
                version();
            }
            "-get" => PrgmMode::Get,
            "-help" => {
                usage(None);
            }
            _ => usage(Some(&args[1])),
        },
        3 => match args[1].as_str() {
            "-set" => {
                to_percent = match args[2].parse::<i16>() {
                    Ok(ival) => ival,
                    Err(_err) => {
                        usage(Some(&args[2]));
                    }
                };

                PrgmMode::Set
            }
            "-inc" => {
                to_percent = match args[2].parse::<i16>() {
                    Ok(ival) => ival,
                    Err(_err) => {
                        usage(Some(&args[2]));
                    }
                };
                PrgmMode::Add
            }
            "-dec" => {
                to_percent = match args[2].parse::<i16>() {
                    Ok(ival) => ival * -1,
                    Err(_err) => {
                        usage(Some(&args[2]));
                    }
                };
                PrgmMode::Add
            }
            _ => usage(Some(&args[1])),
        },
        _ => usage(None),
    };

    let bl_device = match backlight::Backlight::open(None) {
        Ok(bl) => bl,
        Err(err) => {
            eprintln!("Couldn't open backlight device: {}", err);
            std::process::exit(1);
        }
    };

    match mode {
        PrgmMode::Add => {
            let brightness = match bl_device.get_brightness() {
                Ok(brightness) => brightness,
                Err(err) => {
                    eprintln!("Couldn't get brightness: {}", err);
                    std::process::exit(1);
                }
            } as i16;
            if let Err(err) = bl_device.set_brightness((brightness + to_percent) as u16) {
                eprintln!("Couldn't set brightness: {}", err);
            }
        }
        PrgmMode::Get => {
            let brightness = match bl_device.get_brightness() {
                Ok(brightness) => brightness,
                Err(err) => {
                    eprintln!("Couldn't get brightness: {}", err);
                    std::process::exit(1);
                }
            };
            println!("{}", brightness);
        }
        PrgmMode::Set => {
            if let Err(err) = bl_device.set_brightness((to_percent) as u16) {
                eprintln!("Couldn't set brightness: {}", err);
            }
        }
    }
}
