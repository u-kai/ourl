use clap::{Args, Parser, Subcommand};
use std::{collections::BTreeMap, process::Command};

fn main() {
    let cli = Cli::parse();
    cli.run();
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    fn run(&self) {
        match &self.subcmd {
            SubCommand::Shorten(sh) => {
                open(self.short_url(sh).as_str());
            }
            SubCommand::Like(lk) => {
                lk.run();
            }
        }
    }

    fn short_url(&self, sh: &Shorten) -> String {
        if sh.bitly {
            return self.make_url("bit.ly", &sh.path);
        }
        if sh.oreil {
            return self.make_url("oreil.ly", &sh.path);
        }
        match std::env::var("DEFAULT_OURL_DOMAIN") {
            Ok(val) => self.make_url(val.as_str(), &sh.path),
            Err(_) => self.make_url(&sh.domain, &sh.path),
        }
    }

    fn make_url(&self, domain: &str, path: &str) -> String {
        format!("https://{}/{}", domain, path)
    }
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(about = "Open a short URL in the browser.", alias = "short")]
    Shorten(Shorten),
    #[clap(
        about = "Open a browser with the URL alias specified in the configuration file.",
        alias = "lk"
    )]
    Like(Like),
}
#[derive(Args)]
struct Like {
    #[clap(help = "Specify config file path", short = 'c', long = "config")]
    config: Option<String>,
    #[clap(help = "Specify the alias set in the configuration file.")]
    alias: Option<String>,
    #[clap(
        help = "List the aliases in the configuration file.",
        short = 'l',
        long = "list"
    )]
    list: bool,
}

impl Like {
    fn run(&self) {
        if self.list {
            self.list_aliases();
        } else {
            self.open_url();
        }
    }
    fn list_aliases(&self) {
        let config = self.config();
        println!("Aliases:");
        for (alias, url) in config.iter() {
            println!("{}: {}", alias, url);
        }
    }
    fn config(&self) -> BTreeMap<String, String> {
        let default = &format!("{}/.ourl-likes.json", std::env::var("HOME").unwrap());
        let config = self.config.as_ref().unwrap_or(default);

        let config =
            std::fs::read_to_string(config).expect(format!("Failed to read {}", config).as_str());

        serde_json::from_str(config.as_str()).expect("Failed to parse config file.")
    }
    fn open_url(&self) {
        let Some(alias) = &self.alias else {
            panic!("Alias args is required.");
        };
        open(
            &self
                .config()
                .get(alias.as_str())
                .expect(format!("Alias {} not found.", alias).as_str())
                .to_string(),
        )
    }
}
#[derive(Args)]
struct Shorten {
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
    #[clap(
        short = 'o',
        long = "oreil",
        help = "Use oreil.ly to shorten the URL.",
        default_value = "false"
    )]
    oreil: bool,
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
    #[test]
    fn oreil_url_can_specify_o_option() {
        // set other default domain
        std::env::set_var("DEFAULT_OURL_DOMAIN", "example.com");
        let args = vec!["ourl", "Test1", "-o"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.open_url(), "https://oreil.ly/Test1");
        std::env::remove_var("DEFAULT_OURL_DOMAIN");
    }
}
