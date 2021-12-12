use std::io::Read;
use std::time::{Duration, Instant};

use sysinfo::SystemExt;

fn sysproc() {
    let mut t = sysinfo::System::new();
    t.refresh_processes();
}

fn read_files() {
    let mut buf = Vec::new();
    let mut filename = String::new();

    filename.push_str("/proc/");
    let proc_at = filename.len();

    for pid in 1..4000 {
        filename.push_str(&pid.to_string());
        filename.push('/');
        let pid_at = filename.len();
        for f in &["environ", "stat", "status", "cmdline"] {
            filename.push_str(f);
            if let Ok(mut f) = std::fs::File::open(&filename) {
                f.read_to_end(&mut buf).expect("read to end");
            }
            filename.truncate(pid_at);
        }
        filename.truncate(proc_at);
    }
    assert!(buf.capacity() > 2);
}

fn duration<F>(f: F) -> Duration
where
    F: Fn(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_main<F>(f: F, what: &str)
where
    F: Fn(),
{
    eprintln!("\n{} on main: {:?}\n", what, duration(f));
}

fn on_thread<F>(f: F, what: &'static str)
where
    F: Fn() + Send + 'static,
{
    let j = std::thread::spawn(move || {
        eprintln!("\n{} on thread: {:?}\n", what, duration(f));
    });

    let _ = j.join();
}

fn main() {
    // Call with "sys" or "files" to set how /proc is read, or
    // call with "thread" or "main" to set where.
    // e.g. `$0 sys main` or `$0 files thread`.
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let have = |what: &str| args.iter().any(|s| s.contains(&what.to_string()));
    let sys = have("sys");
    let thread = have("th");

    // marker for strace
    let _ = std::fs::File::open("Cargo.toml");

    if sys {
        let what = "sysinfo";
        if thread {
            on_thread(sysproc, what);
        } else {
            on_main(sysproc, what);
        }
    } else {
        let what = "read /proc";
        if thread {
            on_thread(read_files, what);
        } else {
            on_main(read_files, what);
        }
    }
}
