use ::async_std::fs;
use ::async_std::path::Path;

//TODO @mark: get rid of some unwraps?
pub async fn read_file(path: &Path, known_ts_ms: Option<u64>) -> Option<(u64, Vec<u8>)> {
    let meta = fs::metadata(path).await.ok()?;
    let current_ts_ms = meta.modified().unwrap().elapsed().unwrap().as_millis();
    let is_up_to_date = if let Some(known_ts_ms) = known_ts_ms {
        current_ts_ms == (known_ts_ms as u128)
    } else {
        false
    };
    if !is_up_to_date {
        None
    } else {
        let data = fs::read(path).await.unwrap();
        Some((current_ts_ms as u64, data))
    }
}

//TODO @mark: unit test
