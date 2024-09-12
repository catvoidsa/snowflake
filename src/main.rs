use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
const TIME:u64 = 1609459200000;
fn con(pid: u64, mid: u64) -> u64 {
    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("no se que a pasao'")
        .as_millis() as u64;

    let timest: u64 = now - TIME; // 01 01 2021 pal ej.
    let seq: u64 = AtomicU64::new(0).fetch_add(1, Ordering::SeqCst) & 0xFFF;
    let snowflake: u64 = (timest << 22) | (mid << 17) | (pid << 12) | seq;

    snowflake
}

fn dec(snowflake: u64) {
    let timest: u64 = snowflake & 0xFFF;
    let mid: u64 = (snowflake >> 12) & 0x1F;
    let pid: u64 = (snowflake >> 17) & 0x1F;
    let seq: u64 = (snowflake >> 22) + TIME;

    // print pa ejemplo, despues se saca
    println!("time: {}", timest);
    println!("mid: {}", mid);
    println!("pid: {}", pid);
    println!("seq: {}", seq);
}

fn main() {
    let pid: u64 = 1;
    let mid: u64 = 1;

    let snowflake: u64 = con(pid, mid);
    println!("{}", snowflake); // print pq si

    dec(snowflake);
}
