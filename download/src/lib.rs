use curl::easy::Easy;

use std::time::{Duration, Instant};

static URL: &str = "http://localhost:8000/128kib-file";
const TIMEOUT: Duration = Duration::from_secs(0);

pub fn download() -> Result<(), curl::Error> {
    let mut time = Duration::new(0, 0);

    for _ in 0..1000 {
        let mut content = vec![];

        let mut handle = Easy::new();
        handle.url(URL)?;
        handle.timeout(TIMEOUT)?;
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            content.extend_from_slice(data);
            Ok(data.len())
        })?;

        let start = Instant::now();
        transfer.perform()?;
        time += Instant::now() - start;
        drop(transfer);

        assert_eq!(content, vec![0; 128 * 1024]);
    }

    println!("It took {:?}", time);
    Ok(())
}
