use std::thread;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use pprof::ProfilerGuard;
use prost::Message;

fn main() {
    pretty_env_logger::init();
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    bench("dummy");
    profile("bench.pb", guard);
}

fn bench(id: &str) {
    let start = Instant::now();
    let mut count = 0;

    // code here 

    let elapsed_micros = start.elapsed().as_micros();
    let throughput = (count * 1000_000) / elapsed_micros as usize;
    let print = Print {
        id: id.to_owned(),
        message_count: count,
        message_size: 0,
        throughput,
    };

    println!("{}", serde_json::to_string_pretty(&print).unwrap());
    println!("@");
}

#[allow(unused)]
pub fn profile(name: &str, guard: pprof::ProfilerGuard) {
    if let Ok(report) = guard.report().build() {
        let mut file = File::create(name).unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
    };
}

#[derive(Serialize, Deserialize)]
pub struct Print {
    pub id: String,
    pub message_count: usize,
    pub message_size: usize,
    pub throughput: usize,
}
