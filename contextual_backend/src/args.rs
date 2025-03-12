use clap::{Arg, ArgMatches, Command};

pub fn parse_args() -> ArgMatches {
    Command::new("Contextual Backend")
        .arg(
            Arg::new("transport")
                .short('t')
                .long("transport")
                .value_name("TYPE")
                .help("Transport type (unix, tcp, stdio)")
                .required(true)
                .default_value("unix"),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .help("TCP host address")
                .required(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("TCP port")
                .default_value("9876"),
        )
        .arg(
            Arg::new("socket")
                .short('s')
                .long("socke")
                .value_name("PATH")
                .help("Unix socket path")
                .required(true)
                .default_value("/tmp/contextual.sock"),
        )
        .get_matches()
}
