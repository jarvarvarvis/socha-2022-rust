extern crate args;
extern crate getopts;

use std::{env, str::FromStr};

use getopts::Occur;

use args::Args;

use crate::util::error::Error;

#[derive(Debug, Clone)]
pub struct ClientArgs {
    pub host: String,
    pub port: i32,
    pub reservation: Option<String>,
}

impl ClientArgs {
    fn setup_args(program: &str) -> Args {
        let mut args = Args::new(
            program,
            "A client for the game Ostseeschach (Software Challenge 2022)",
        );
        args.option(
            "h",
            "host",
            "The IP address of the host to connect to",
            "HOST",
            Occur::Req,
            None,
        );
        args.option(
            "p",
            "port",
            "The port used for the connection",
            "PORT",
            Occur::Req,
            None,
        );
        args.option(
            "r",
            "reservation",
            "The reservation code required for joining a prepared game.",
            "CODE",
            Occur::Optional,
            None,
        );

        args
    }

    fn evaluate_argument<T>(args: &Args, name: &str) -> Result<T, Error>
    where
        T: FromStr,
    {
        let args_value = args.value_of::<T>(name);
        match args_value {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::ArgsError(error)),
        }
    }

    fn evaluate_opt_argument<T>(args: &Args, name: &str) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        let args_value = args.optional_value_of::<T>(name);
        match args_value {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::ArgsError(error)),
        }
    }

    fn create_client_args(args: &Args) -> Result<Self, Error> {
        let host = Self::evaluate_argument::<String>(args, "host")?;
        let port_string = Self::evaluate_argument::<String>(args, "port")?;
        let reservation = Self::evaluate_opt_argument::<String>(args, "reservation")?;

        let port_parse_result = port_string.parse::<i32>();
        match port_parse_result {
            Ok(port) => Ok(Self {
                host,
                port,
                reservation,
            }),
            Err(error) => Err(Error::ParseIntError(error)),
        }
    }

    pub fn collect() -> Result<Self, Error> {
        let env_args: Vec<String> = env::args().collect();
        let program = env_args.first().unwrap();

        let mut args = Self::setup_args(program);

        let parse_result = args.parse(env_args);

        if parse_result.is_err() {
            let usage = args.full_usage();
            println!("{}", usage);

            let error = parse_result.unwrap_err();
            return Err(Error::ArgsError(error));
        }

        Self::create_client_args(&args)
    }
}
