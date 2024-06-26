use clap::Parser;
use std::process::Command;

fn main() {
    let cli = Cli::parse();
    cli.run();
}

#[derive(Parser)]
struct Cli {
    #[clap(
        short = 'd',
        long = "domain",
        default_value = "oreil.ly",
        help = r#"The domain name to open in the browser. You can override this by setting the `DEFAULT_OURL_DOMAIN` environment variable."#
    )]
    domain: String,
    #[clap(help = "The URL path to open in the browser.")]
    path: String,
    #[clap(
        short = 'b',
        long = "bitly",
        help = "Use bit.ly to shorten the URL.",
        default_value = "false"
    )]
    bitly: bool,
}

impl Cli {
    fn run(&self) {
        open(self.open_url().as_str());
    }
    fn open_url(&self) -> String {
        if self.bitly {
            return self.make_url("bit.ly");
        }
        match std::env::var("DEFAULT_OURL_DOMAIN") {
            Ok(val) => self.make_url(val.as_str()),
            Err(_) => self.make_url(&self.domain),
        }
    }
    fn make_url(&self, domain: &str) -> String {
        format!("https://{}/{}", domain, self.path)
    }
}

#[cfg(target_os = "macos")]
fn open(url: &str) {
    Command::new("open")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}

#[cfg(target_os = "linux")]
fn open(url: &str) {
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}

#[cfg(target_os = "windows")]
fn open(url: &str) {
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(url)
        .spawn()
        .expect(format!("Failed to open {}", url).as_str());
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;
    #[test]
    fn open_url_default_is_oreil_ly() {
        let args = vec!["ourl", "Test1"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://oreil.ly/Test1");
    }

    #[test]
    fn open_url_can_specify_domain() {
        let args = vec!["ourl", "Test1", "-d", "example.com"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://example.com/Test1");
    }

    #[test]
    fn open_url_can_specify_default_domain_use_by_env() {
        std::env::set_var("DEFAULT_OURL_DOMAIN", "example.com");
        let args = vec!["ourl", "Test1"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://example.com/Test1");
        std::env::remove_var("DEFAULT_OURL_DOMAIN");
    }
    #[test]
    fn bitly_url_can_specify_b_option() {
        let args = vec!["ourl", "Test1", "-b"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://bit.ly/Test1");
    }
}
