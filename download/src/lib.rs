use curl::easy::Easy;

use std::time::{Duration, Instant};

static URL: &str = "https://file-examples.com/wp-content/uploads/2017/02/zip_2MB.zip";
const TIMEOUT: Duration = Duration::from_secs(0);

pub fn download() -> Result<(), curl::Error> {
    let mut len = 0;
    let mut handle = Easy::new();
    handle.url(URL)?;
    handle.timeout(TIMEOUT)?;
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

    assert_eq!(len, 2 * 1024usize.pow(2));

    println!("It took {:?}", time);
    Ok(())
}
