use lab05::canvas::Canvas;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn oldest_and_youngest(students: &[Student]) -> Result<(&Student, &Student), &'static str> {
    let oldest = students
        .iter()
        .max_by_key(|student| student.age)
        .ok_or("the input contains no students")?;
    let youngest = students
        .iter()
        .min_by_key(|student| student.age)
        .expect("oldest_and_youngest checked that students is not empty");
    Ok((oldest, youngest))
}

fn print_age_extremes(students: &[Student]) -> Result<(), Box<dyn Error>> {
    let (oldest, youngest) = oldest_and_youngest(students)?;
    println!(
        "Oldest student: {} ({}), age {}",
        oldest.name, oldest.phone, oldest.age
    );
    println!(
        "Youngest student: {} ({}), age {}",
        youngest.name, youngest.phone, youngest.age
    );
    Ok(())
}

fn read_csv_students(path: &Path) -> Result<Vec<Student>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut students = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<_> = line.split(',').map(str::trim).collect();
        if fields.len() != 3 {
            return Err(format!(
                "{}:{}: expected name,phone,age",
                path.display(),
                line_index + 1
            )
            .into());
        }

        let age = fields[2].parse::<u32>().map_err(|error| {
            format!(
                "{}:{}: invalid age: {error}",
                path.display(),
                line_index + 1
            )
        })?;
        students.push(Student {
            name: fields[0].to_owned(),
            phone: fields[1].to_owned(),
            age,
        });
    }
    Ok(students)
}

fn read_json_students(path: &Path) -> Result<Vec<Student>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut students = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let student = serde_json::from_str::<Student>(line).map_err(|error| {
            format!(
                "{}:{}: invalid student JSON: {error}",
                path.display(),
                line_index + 1
            )
        })?;
        students.push(student);
    }
    Ok(students)
}

fn canvas_demo() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new();
    for (x, y, value) in [(2, 2, b'R'), (3, 2, b'u'), (4, 2, b's'), (5, 2, b't')] {
        canvas.set_pixel(x, y, value)?;
    }
    canvas.print();
    Ok(())
}

fn usage() {
    eprintln!(
        "Usage:\n  lab05 p1 <students.csv>\n  lab05 p2\n  lab05 p3 <students.jsonl>\n\nP2 prints a 100x55 canvas with a small demo drawing."
    );
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("p1") if args.len() == 3 => {
            let students = read_csv_students(Path::new(&args[2]))?;
            print_age_extremes(&students)
        }
        Some("p2") if args.len() == 2 => canvas_demo(),
        Some("p3") if args.len() == 3 => {
            let students = read_json_students(Path::new(&args[2]))?;
            print_age_extremes(&students)
        }
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
