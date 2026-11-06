use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::time::Instant;

const BUFFER_SIZE: usize = 1024 * 1024;
const FOUR_GIB: u64 = 4 * 1024 * 1024 * 1024;
const REPEATED_TEXT: &[u8] = b"Rust makes text processing fun!\n";

fn longest_lines(path: &Path) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut longest_by_bytes: Option<(&str, usize)> = None;
    let mut longest_by_chars: Option<(&str, usize)> = None;

    for line in text.lines() {
        let byte_count = line.len();
        let char_count = line.chars().count();
        if longest_by_bytes.is_none_or(|(_, max)| byte_count > max) {
            longest_by_bytes = Some((line, byte_count));
        }
        if longest_by_chars.is_none_or(|(_, max)| char_count > max) {
            longest_by_chars = Some((line, char_count));
        }
    }

    match longest_by_bytes {
        Some((line, count)) => println!("Longest line by bytes ({count}): {line}"),
        None => println!("Longest line by bytes: <no lines>"),
    }
    match longest_by_chars {
        Some((line, count)) => println!("Longest line by characters ({count}): {line}"),
        None => println!("Longest line by characters: <no lines>"),
    }
    Ok(())
}

fn rot13_byte(byte: u8) -> u8 {
    match byte {
        b'a'..=b'z' => (byte - b'a' + 13) % 26 + b'a',
        b'A'..=b'Z' => (byte - b'A' + 13) % 26 + b'A',
        _ => byte,
    }
}

fn rot13_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let canonical_input = fs::canonicalize(input_path)?;
    let output_parent = output_path.parent().unwrap_or_else(|| Path::new("."));
    let output_name = output_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "output must be a file path"))?;
    let canonical_output = if output_path.exists() {
        fs::canonicalize(output_path)?
    } else {
        fs::canonicalize(output_parent)?.join(output_name)
    };
    if canonical_input == canonical_output {
        return Err("input and output paths must be different".into());
    }

    let input = File::open(input_path)?;
    let output = File::create(output_path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, input);
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, output);
    let mut buffer = vec![0; BUFFER_SIZE];

    let mut bytes_read = 0_u64;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        for (index, byte) in buffer[..count].iter_mut().enumerate() {
            if !byte.is_ascii() {
                return Err(format!(
                    "non-ASCII byte found near input byte {}",
                    bytes_read + index as u64
                )
                .into());
            }
            *byte = rot13_byte(*byte);
        }
        writer.write_all(&buffer[..count])?;
        bytes_read += count as u64;
    }
    writer.flush()?;
    Ok(())
}

fn expand_abbreviations(path: &Path) -> Result<(), Box<dyn Error>> {
    let phrase = fs::read_to_string(path)?;
    let expanded = phrase
        .split(' ')
        .map(|word| match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        })
        .collect::<Vec<_>>()
        .join(" ");
    print!("{expanded}");
    Ok(())
}

fn hosts_path() -> &'static str {
    if cfg!(windows) {
        r"C:\Windows\System32\drivers\etc\hosts"
    } else {
        "/etc/hosts"
    }
}

fn print_hosts(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut columns = line.split_whitespace();
        if let (Some(address), Some(hostname)) = (columns.next(), columns.next()) {
            println!("{hostname} => {address}");
        }
    }
    Ok(())
}

fn generate_four_gib(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, file);
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    while buffer.len() + REPEATED_TEXT.len() <= BUFFER_SIZE {
        buffer.extend_from_slice(REPEATED_TEXT);
    }
    let complete_buffers = FOUR_GIB / buffer.len() as u64;
    let remainder = (FOUR_GIB % buffer.len() as u64) as usize;
    for _ in 0..complete_buffers {
        writer.write_all(&buffer)?;
    }
    writer.write_all(&buffer[..remainder])?;
    writer.flush()?;
    println!("Generated exactly {FOUR_GIB} bytes at {}", path.display());
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage:\n  lab04 p1 <input>\n  lab04 p2 <input> <output>\n  lab04 p3 <input>\n  lab04 p4 [hosts-file]\n  lab04 generate-4gib <output>\n\nP2 reports the elapsed time for ROT13. The output may be partial if a non-ASCII byte or I/O error occurs."
    );
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("p1") if args.len() == 3 => longest_lines(Path::new(&args[2])),
        Some("p2") if args.len() == 4 => {
            let start = Instant::now();
            rot13_file(Path::new(&args[2]), Path::new(&args[3]))?;
            println!("ROT13 completed in {:?}", start.elapsed());
            Ok(())
        }
        Some("p3") if args.len() == 3 => expand_abbreviations(Path::new(&args[2])),
        Some("p4") if args.len() <= 3 => {
            let path = args.get(2).map(String::as_str).unwrap_or(hosts_path());
            print_hosts(Path::new(path))
        }
        Some("generate-4gib") if args.len() == 3 => generate_four_gib(Path::new(&args[2])),
        _ => {
            usage();
            Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid command or arguments").into())
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
