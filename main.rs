enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}
#[derive(Debug)]
struct Size {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Size {
    fn new() -> Self {
        Self {
            bytes: "".to_string(),
            kilobytes: "".to_string(),
            megabytes: "".to_string(),
            gigabytes: "".to_string(),
        }
    }
    fn formatter(&mut self, size: FileSize) {
        match size {
            FileSize::Bytes(bytes) => {
                self.bytes = format!("{} bytes", bytes);
                self.kilobytes = format!("{:.2} KB", bytes as f64 / 1000.0);
                self.megabytes = format!("{:.2} MB", bytes as f64 / 1_000_000.0);
                self.gigabytes = format!("{:.2} GB", bytes as f64 / 1_000_000_000.0);
            }
            FileSize::Kilobytes(kb) => {
                self.bytes = format!("{} bytes", kb * 1000.0);
                self.kilobytes = format!("{:.2} KB", kb);
                self.megabytes = format!("{:.2} MB", kb / 1000.0);
                self.gigabytes = format!("{:.2} GB", kb / 1_000_000.0);
            }
            FileSize::Megabytes(mb) => {
                self.bytes = format!("{} bytes", mb * 1_000_000.0);
                self.kilobytes = format!("{:.2} KB", mb * 1000.0);
                self.megabytes = format!("{:.2} MB", mb);
                self.gigabytes = format!("{:.2} GB", mb / 1000.0);
            }
            FileSize::Gigabytes(gb) => {
                self.bytes = format!("{} bytes", gb * 1_000_000_000.0);
                self.kilobytes = format!("{:.2} KB", gb * 1_000_000.0);
                self.megabytes = format!("{:.2} MB", gb * 1_000.0);
                self.gigabytes = format!("{:.2} GB", gb);
            }
        }
    }
}

fn format_input_size(size: u64, unit: String) -> FileSize {
    let filesize = match unit.as_str() {
        "B" => FileSize::Bytes(size),
        "KB" => FileSize::Kilobytes(size as f64),
        "MB" => FileSize::Megabytes(size as f64),
        "GB" => FileSize::Gigabytes(size as f64),
        _ => FileSize::Bytes(size),
    };
    return filesize;
}

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let result_vec = inp.split(" ").collect::<Vec<&str>>();
    let size = result_vec[0]
        .replace(" ", "")
        .trim()
        .to_string()
        .parse::<u64>()
        .unwrap();
    let unit = result_vec[1]
        .replace(" ", "")
        .trim()
        .to_string()
        .to_uppercase();
    let file_size_format = format_input_size(size, unit);
    let mut sizer = Size::new();
    sizer.formatter(file_size_format);
    let result = sizer;

    println!("{:?}", result);
}
