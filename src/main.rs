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
        help = r#"The domain name to open in the browser.
If you set specific default value, you set environment variable `DEFAULT_SURL_DOMAIN` to override it.
"#
    )]
    domain: String,
    #[clap(help = "The url path to open in the browser.")]
    path: String,
}

impl Cli {
    fn run(&self) {
        open(self.open_url().as_str());
    }

    fn open_url(&self) -> String {
        let domain = match std::env::var("DEFAULT_SURL_DOMAIN") {
            Ok(val) => val,
            Err(_) => self.domain.clone(),
        };
        format!("https://{}/{}", domain, self.path)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_url_default_is_oreil_ly() {
        let args = vec!["surl", "Test1"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://oreil.ly/Test1");
    }

    #[test]
    fn open_url_can_change_use_by_specific_domain() {
        let args = vec!["surl", "Test1", "-d", "example.com"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://example.com/Test1");
    }

    #[test]
    #[serial_test::serial]
    fn open_url_can_change_use_by_env() {
        std::env::set_var("DEFAULT_SURL_DOMAIN", "example.com");
        let args = vec!["surl", "Test1"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://example.com/Test1");
        std::env::remove_var("DEFAULT_SURL_DOMAIN");
    }
}
