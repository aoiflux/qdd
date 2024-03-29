use nix::sys::sendfile;
use std::{
    env, fs,
    io::{stdout, Write},
    os::fd::AsFd,
    process::{self, Command},
};

const BUFF_SIZE: usize = 4096 * 1000 * 1000;

fn main() {
    let args = get_iops();
    let size = get_size(&args[1]);
    image(size, &args[1], &args[2]);
    println!("Done!");
}

fn image(size: u64, infpath: &String, outfpath: &String) {
    let from_fd = fs::OpenOptions::new().read(true).open(infpath).unwrap();
    let to_fd = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(outfpath)
        .unwrap();

    let out_fd = to_fd.as_fd();
    let in_fd = from_fd.as_fd();

    let mut offset: i64 = 0;

    for i in (0..size).step_by(BUFF_SIZE) {
        print!("\rImaging: {:} ", (i as f32 / size as f32) * (100 as f32));
        stdout().flush().unwrap();
        let result = sendfile::sendfile64(out_fd, in_fd, Some(&mut offset), BUFF_SIZE);
        match result {
            Ok(0) => break,
            Ok(_) => continue,
            Err(err) => {
                print!("{}\n", err);
                break;
            }
        }
    }
}

fn get_size(infpath: &String) -> u64 {
    let out = Command::new("blockdev")
        .arg("--getsize64")
        .arg(infpath)
        .output()
        .expect("somethingw went wong");

    let size_str = String::from_utf8_lossy(&out.stdout);
    let size: u64 = size_str
        .trim()
        .parse()
        .expect("Failed to parse string to u64");
    return size;
}

fn get_iops() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if (args.len()) < 2 {
        println!("Usage: qdd <infile> <outfile>");
        process::exit(1);
    }
    return args;
}
