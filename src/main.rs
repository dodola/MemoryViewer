use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::str::FromStr;

fn read_process_memory(pid: i32, address: usize, size: usize) -> io::Result<Vec<u8>> {
    let mem_path = format!("/proc/{}/mem", pid);
    let mut file = File::open(mem_path)?;

    // Seek to the start address
    file.seek(SeekFrom::Start(address as u64))?;

    // Read the memory contents into a buffer
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn print_memory_contents(address: usize, bytes: &[u8]) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let mut hex_part = String::new();
        let mut ascii_part = String::new();

        for &byte in chunk {
            hex_part.push_str(&format!("{:02X} ", byte));
            ascii_part.push(if byte.is_ascii_graphic() || byte == b' ' {
                char::from(byte)
            } else {
                '.'
            });
        }

        println!("0x{:016X} | {:48} | {}", address + i * 16, hex_part, ascii_part);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Expecting arguments in the order of: PID, address, size
    if args.len() < 4 {
        println!("Usage: <program> <pid> <address> <size>");
        return;
    }

    let pid = i32::from_str(&args[1]).expect("Error: PID must be an integer.");
    let address = usize::from_str_radix(&args[2].trim_start_matches("0x"), 16)
        .expect("Error: Address must be a hexadecimal number.");
    let size = usize::from_str(&args[3]).expect("Error: Size must be an integer.");

    match read_process_memory(pid, address, size) {
        Ok(bytes) => {
            print_memory_contents(address, &bytes);
        }
        Err(e) => {
            println!("Failed to read memory: {}", e);
        }
    }
}