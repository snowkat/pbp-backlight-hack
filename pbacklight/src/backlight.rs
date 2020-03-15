use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const BACKLIGHT_DIR: &str = "/sys/class/backlight";

pub struct Backlight {
    path: PathBuf,
}

impl Backlight {
    fn get_value(&self, entry: &str) -> Result<u32, Box<dyn Error>> {
        let mut attrib = String::new();
        File::open(self.path.join(entry))?.read_to_string(&mut attrib)?;

        let attrib = attrib.trim().parse::<u32>()?;
        Ok(attrib)
    }

    fn set_value(&self, entry: &str, value: u32) -> Result<(), Box<dyn Error>> {
        let mut attrib_file = OpenOptions::new()
            .write(true)
            .create(false)
            .open(self.path.join(entry))?;

        write!(&mut attrib_file, "{}", value)?;
        Ok(())
    }

    pub fn open(backlight_name: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let backlight_path = {
            let sys_path = Path::new(BACKLIGHT_DIR);
            match backlight_name {
                Some(name) => Ok(sys_path.join(name)),
                None => match sys_path.read_dir()?.nth(0) {
                    Some(dir) => Ok(dir?.path()),
                    None => Err(std::io::Error::new(
                        ErrorKind::NotFound,
                        "No backlights found",
                    )),
                },
            }
        }?;

        Ok(Backlight {
            path: backlight_path,
        })
    }

    pub fn get_brightness(&self) -> Result<u16, Box<dyn Error>> {
        let max_brightness = self.get_value("max_brightness")?;
        let curr_brightness = self.get_value("brightness")?;
        let bright_percent = ((curr_brightness as f32) / (max_brightness as f32)) * 100.0;
        Ok(bright_percent.round() as u16)
    }

    pub fn set_brightness(&self, percent: u16) -> Result<(), Box<dyn Error>> {
        let max_brightness = self.get_value("max_brightness")?;
        let steps = (max_brightness as f32) / 100.0;
        let final_brightness = ((percent as f32) * steps).round() as u32;
        self.set_value("brightness", final_brightness)
    }
}
