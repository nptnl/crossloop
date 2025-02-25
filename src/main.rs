use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path: String = args[1].clone();
    if args.len() < 3 { hourly(path, 1.0); return (); }
    let volume: f32 = match args[2].parse::<f32>() {
        Ok(v) => v,
        Err(_) => 1.0,
    };
    hourly(path, volume);
}
fn app_one(queue: &Sink, path: String) -> Result<i32, i32> {
    let file;
    match File::open(path.as_str()) {
        Err(_) => return Err(2),
        Ok(v) => file = BufReader::new(v),
    }
    match Decoder::new(file) {
        Err(_) => return Err(3),
        Ok(v) => queue.append(v),
    }
    return Ok(0);
}

fn hourly(path: String, volume: f32) {
    let (_mogus, sthand) = OutputStream::try_default().unwrap();
    let sinky = Sink::try_new(&sthand).unwrap();
    sinky.set_volume(volume);

    let mut hour = (time::UNIX_EPOCH.elapsed().unwrap().as_secs()) % 86400 / 3600;
    hour += 24 - 5;
    hour %= 24;
    let mut second: u64 = time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 3600;
    let mut hrpath: String;
    let mut runpath: String;

    loop {
        hrpath = hour_name(path.clone(), hour);
        runpath = hrpath.clone() + "-start.wav";
        app_one(&sinky, runpath);
        
        runpath = hrpath + "-loop.wav";
        sinky.play();
        until_hour(&sinky, runpath, 3600 - second);
        if hour == 24 { hour = 0 } else { hour += 1 };
        second = time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 3600 - 3600;
    }
}
fn hour_name(path: String, hour: u64) -> String {
    if hour < 10 {
        format!("{path}/0{hour}")
    } else {
        format!("{path}/{hour}")
    }
}
fn until_hour(queue: &Sink, path: String, remaining: u64) -> Result<i32, i32> {
    let file;
    match File::open(path.as_str()) {
        Err(_) => return Err(2),
        Ok(v) => file = BufReader::new(v),
    }
    let sorc;
    match Decoder::new(file) {
        Err(_) => return Err(3),
        Ok(v) => sorc = v,
    }
    let sorctime;
    match sorc.total_duration() {
        None => { queue.append(sorc); return Ok(1) },
        Some(v) => sorctime = v.as_secs(),
    }
    for _ in 1..(remaining / sorctime) {
        let file;
        match File::open(path.as_str()) {
            Err(_) => return Err(2),
            Ok(v) => file = BufReader::new(v),
        }
        let subsorq;
        match Decoder::new(file) {
            Err(_) => return Err(3),
            Ok(v) => subsorq = v,
        }
        queue.append(subsorq);
        std::thread::sleep(time::Duration::from_secs(sorctime));
    }
    return Ok(0);
}
