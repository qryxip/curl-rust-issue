use curl::easy::Easy;

use std::time::{Duration, Instant};

static URL: &str = "https://file-examples.com/wp-content/uploads/2017/02/zip_2MB.zip";
const TIMEOUT: Duration = Duration::from_secs(20);
const LOW_SPEED_LIMIT: u32 = 1;
const LOW_SPEED_TIME: Duration = Duration::from_secs(30);

pub fn download() -> Result<(), curl::Error> {
    let mut len = 0;
    let mut handle = Easy::new();
    handle.url(URL)?;
    handle.timeout(TIMEOUT)?;
    handle.low_speed_limit(LOW_SPEED_LIMIT)?;
    handle.low_speed_time(LOW_SPEED_TIME)?;
    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        len += data.len();
        Ok(data.len())
    })?;
    transfer.perform()?;

    let start = Instant::now();
    transfer.perform()?;
    let time = Instant::now() - start;
    drop(transfer);

    assert_eq!(len, 2_097_152);

    println!("It took {:?}", time);
    Ok(())
}
