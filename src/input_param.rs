use std::io;

#[derive(Debug)]
pub enum ParsedCommand {
    Start(String, String, String),
    Stop(String),
    Slow(String, String),
    Resume(String),
}

impl ParsedCommand {
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
    match input.next().as_deref() {
        Some("start") => parse_start(input),
        Some("stop") => parse_stop(input),
        Some("slow") => parse_slow(input),
        Some("resume") => parse_resume(input),
        Some(cmd) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unknown command: {}", cmd),
        )),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot parse command",
        )),
    }
}

fn parse_start(mut input: impl Iterator<Item = String>) -> io::Result<ParsedCommand> {
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

    Ok(ParsedCommand::Start(
        name,
        expose.to_string(),
        target.to_string(),
    ))
}

fn parse_stop(mut input: impl Iterator<Item = String>) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;

    Ok(ParsedCommand::Stop(name))
}

fn parse_slow(mut input: impl Iterator<Item = String>) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;
    let time = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing time argument"))?;

    Ok(ParsedCommand::Slow(name, time))
}

fn parse_resume(mut input: impl Iterator<Item = String>) -> io::Result<ParsedCommand> {
    let name = input
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing name argument"))?;

    Ok(ParsedCommand::Resume(name))
}
