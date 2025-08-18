use std::io;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    pub fn from_str(request: &str) -> io::Result<Self> {
        parse_args_from_string(request.to_string())
    }
}

// Example usage:
// start my_app_https 1001:127.0.0.1:7176
// stop my_app_https
// slow my_app_https 200
pub fn parse_args_from_string(input: String) -> io::Result<ParsedCommand> {
    let split = input.split(" ").map(|x| x.to_string());
    parse_args_from_iterator(split)
}

pub fn parse_args_from_iterator(
    mut input: impl Iterator<Item = String>,
) -> io::Result<ParsedCommand> {
    let result = match input.next() {
        Some(command) => match command.as_str() {
            "start" => match parse_start(command, input) {
                Ok(x) => Some(x),
                _ => None,
            },
            "stop" => match parse_stop(command, input) {
                Ok(x) => Some(x),
                _ => None,
            },
            "slow" => match parse_slow(command, input) {
                Ok(x) => Some(x),
                _ => None,
            },
            _ => None,
        },
        None => None,
    };

    match result {
        Some(x) => Ok(x),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot parse command",
        )),
    }
}

fn parse_start(
    command: String,
    mut input: impl Iterator<Item = String>,
) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
    let address_pair = input.next().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Missing address pair argument")
    })?;
    let separator = address_pair.find(":").ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Missing ':' in address pair")
    })?;
    let expose = &address_pair[..separator];
    let target = &address_pair[separator + 1..];
    let args = [name, expose.to_string(), target.to_string()];
    Ok(ParsedCommand::new(command, args.to_vec()))
}

fn parse_stop(
    command: String,
    mut input: impl Iterator<Item = String>,
) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
    let args = [name];
    Ok(ParsedCommand::new(command, args.to_vec()))
}

fn parse_slow(
    command: String,
    mut input: impl Iterator<Item = String>,
) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
    let time = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing time argument"))?;
    let args = [name, time];
    Ok(ParsedCommand::new(command, args.to_vec()))
}
