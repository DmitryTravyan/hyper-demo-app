use clap::{App, SubCommand, Arg};

struct HyperApp();

impl HyperApp {
    fn new() -> Self {
        let args = App::new("")
            .version("")
            .about("")
            .author("")
            .subcommands(vec![
                SubCommand::with_name("start")
                    .about("")
                    .help("")
                    .args(&[
                        Arg::with_name("hostname")
                            .help("")
                            .long("hostname")
                            .env("HOSTNAME")
                            .takes_value(true),
                        Arg::with_name("port")
                            .help("")
                            .long("port")
                            .env("PORT")
                            .takes_value(true)
                    ])
            ])
            .get_matches();
        Self()
    }
}