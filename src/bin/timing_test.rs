use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let mut ms_loops = 0;

    loop {
        let current_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        if current_ms == ms {
            ms_loops += 1;
        } else {
            println!("Previous millisecond {} was run {} times", ms, ms_loops);
            ms = current_ms;
            ms_loops = 0;
        }
    }
}
