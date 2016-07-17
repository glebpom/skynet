extern crate coio;
extern crate time;

use coio::Scheduler;


fn skynet(num: u32, size: u32, div: u8) -> u64 {

    if size == 1 {
        return num as u64;
    }
    let mut sum = 0u64;

    let mut v = Vec::with_capacity(div as usize);

    for i in 0..div {
        let sd: u32 = size / div as u32;
        let sub_num: u32 = num + i as u32 * sd;
        v.push(Scheduler::spawn(move || -> u64 {
            skynet(sub_num, sd, div)
        }));
    }

    for c in v {
        sum += c.join().unwrap()
    }
    sum
}

fn main() {
    let started_at = time::get_time();
    let result = Scheduler::new().with_workers(4)
        .run(|| {
            skynet(0, 1000000, 10)
        })
        .unwrap();
    let duration = time::get_time() - started_at;
    println!("Result {} in {} ms.", result, duration.num_milliseconds());
}
