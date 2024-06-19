use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.run();
}

#[derive(clap::Parser)]
struct Cli {
    #[clap(
        short = 'd',
        long = "domain",
        default_value = "oreil.ly",
        help = "The domain name to open in the browser.\n If not provided, the default is oreil.ly.\nIf you set specific default value, you set environment variable `DEFAULT_DOMAIN` to override it."
    )]
    domain: String,
    path: String,
}

impl Cli {
    fn run(&self) {
        let url = format!("https://{}/{}", self.domain, self.path);
        open(&url);
    }
}

#[cfg(target_os = "macos")]
fn open(url: &str) {
    use std::process::Command;
    Command::new("open")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}

#[cfg(target_os = "linux")]
fn open(url: &str) {
    use std::process::Command;
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}

#[cfg(target_os = "windows")]
fn open(url: &str) {
    use std::process::Command;
    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}
