use anyhow::Result;

const PREFIX: &str = "/sys/class/power_supply/BAT0";
const MIN: i32 = 60;
const MAX: i32 = 80;
const SLEEP: u64 = 2;

fn set_thresh(v: i32) -> Result<()> {
    println!("NEW_THRESH {v}");
    std::fs::write(
        format!("{PREFIX}/charge_control_end_threshold"),
        format!("{v}"),
    )?;
    Ok(())
}

fn my_read_to_string<P>(p: P) -> Result<String>
where
    P: AsRef<std::path::Path>,
{
    Ok(std::fs::read_to_string(p)?.trim().to_string())
}

fn tick() -> Result<()> {
    let status = my_read_to_string(format!("{PREFIX}/status"))?;
    let cur_thresh =
        my_read_to_string(format!("{PREFIX}/charge_control_end_threshold"))?.parse::<i32>()?;
    let new_thresh = if cur_thresh == 100 || status == "Discharging" {
        let level = my_read_to_string(format!("{PREFIX}/capacity"))?.parse::<i32>()?;
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

fn main() -> Result<()> {
    println!("starting rpbatmanager v{}", env!("CARGO_PKG_VERSION"));
    println!("PREFIX {PREFIX}");
    if std::env::args().any(|x| x == "full") {
        set_thresh(99)?;
    } else if let Some(x) = std::env::args().nth(1) {
        let val = x.parse()?;
        let val = if val == 100 { 99 } else { val };
        set_thresh(val)?;
    } else if std::env::args().len() > 1 {
        anyhow::bail!("weird arguments")
        //return anyhow::anyhow!("weird arguments");
    } else {
        let dur = std::time::Duration::from_secs(SLEEP);
        loop {
            tick()?;
            std::thread::sleep(dur);
        }
    }
    Ok(())
}
