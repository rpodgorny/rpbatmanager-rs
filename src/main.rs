use std::error::Error;

const PREFIX: &str = "/sys/class/power_supply/BAT0";
const MIN: i32 = 60;
const MAX: i32 = 80;
const SLEEP: u64 = 2;

fn set_thresh(v: i32) -> Result<(), Box<dyn Error>> {
    println!("NEW_THRESH {v}");
    std::fs::write(
        format!("{PREFIX}/charge_control_end_threshold"),
        format!("{v}"),
    )?;
    Ok(())
}

fn my_read_to_string<P>(p: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<std::path::Path>,
{
    Ok(std::fs::read_to_string(p)?.trim().to_string())
}

fn tick() -> Result<(), Box<dyn Error>> {
    let status = my_read_to_string(format!("{PREFIX}/status"))?;
    let level = my_read_to_string(format!("{PREFIX}/capacity"))?.parse::<i32>()?;
    let cur_thresh = my_read_to_string(format!("{PREFIX}/charge_control_end_threshold"))?.parse::<i32>()?;
    let new_thresh = if cur_thresh == 100 || status == "Discharging" {
        Some(if level < MIN { MAX } else { MIN })
    } else {
        None
    };
    //println!("STATUS {status} {level} {thresh} {new_thresh:?}");
    if let Some(new_thresh) = new_thresh {
        if new_thresh != cur_thresh {
            set_thresh(new_thresh)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("starting rpbatmanager v{}", env!("CARGO_PKG_VERSION"));
    println!("PREFIX {PREFIX}");
    if std::env::args()
        .find(|x| *x == "full".to_string())
        .is_some()
    {
        set_thresh(99)?;
    } else {
        let dur = std::time::Duration::from_secs(SLEEP);
        loop {
            tick()?;
            std::thread::sleep(dur);
        }
    }
    Ok(())
}
