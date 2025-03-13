use clap::{Error, Parser, ValueEnum, error::ErrorKind};

#[derive(Parser)]
#[command(name = "Contextual Backend")]
pub struct Args {
    /// Transport type (unix, tcp, stdio)
    #[arg(
        short = 't',
        long,
        value_name = "TYPE",
        default_value_t = Transport::Stdio
    )]
    transport: Transport,

    /// TCP host address (required when transport is tcp)
    #[arg(long, value_name = "HOST")]
    host: Option<String>,

    /// TCP port (required when transport is tcp)
    #[arg(short = 'p', long, value_name = "PORT")]
    port: Option<u16>,

    /// Unix socket path (required when transport is unix)
    socket: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Transport {
    Unix,
    Tcp,
    Stdio,
}

impl std::fmt::Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transport_type = match self {
            Transport::Unix => "unix",
            Transport::Tcp => "tcp",
            Transport::Stdio => "stdio",
        };
        write!(f, "{}", transport_type)
    }
}

impl Args {
    /// Parse arguments from the command line and validate argument
    /// combinations depending on the [TransportType].
    pub fn parse_and_validate() -> ValidatedArgs {
        let args = Self::parse();
        if let Err(e) = args.validate() {
            e.exit();
        }

        ValidatedArgs {
            // we can safely unwrap here since the values have been validated
            // before this
            transport: match args.transport {
                Transport::Unix => TransportType::Unix {
                    socket_path: args.socket.unwrap(),
                },
                Transport::Tcp => TransportType::Tcp {
                    host: args.host.unwrap(),
                    port: args.port.unwrap(),
                },
                Transport::Stdio => TransportType::Stdio,
            },
        }
    }

    /// Validate arguments base on transport type
    fn validate(&self) -> Result<(), clap::Error> {
        match self.transport {
            Transport::Unix => {
                if self.socket.is_none() {
                    return Err(Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The argument '--socket <PATH>' is required when transport is 'unix'",
                    ));
                }
            }
            Transport::Tcp => {
                if self.host.is_none() {
                    return Err(Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The argument --host <HOST> is required when transport is 'tcp'",
                    ));
                }

                if self.port.is_none() {
                    return Err(Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The argument --port <PORT> is required when transport is 'tcp'",
                    ));
                }
            }
            Transport::Stdio => {}
        }

        Ok(())
    }
}

pub struct ValidatedArgs {
    pub transport: TransportType,
}

pub enum TransportType {
    Unix { socket_path: String },
    Tcp { host: String, port: u16 },
    Stdio,
}
