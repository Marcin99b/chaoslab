use std::io;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    pub fn new(name: String, args: Vec<String>) -> ParsedCommand {
        ParsedCommand { name, args }
    }
}

/// example
///
/// start my_app_https 1001:127.0.0.1:7176
/// stop my_app_https
/// slow my_app_https 200ms
///
/// app_name=expose_port:target_address
pub fn parse_args_from_string(input: String) -> io::Result<ParsedCommand> {
    let split = input.split(" ").map(|x| x.to_string());
    parse_args_from_iterator(split)
}

//todo add validation
pub fn parse_args_from_iterator(
    mut input: impl Iterator<Item = String>,
) -> io::Result<ParsedCommand> {
    let result = match input.next() {
        Some(command) => match command.as_str() {
            "start" => {
                let name = input.next().unwrap();
                let address_pair = input.next().unwrap();
                let separator = address_pair.find(":").unwrap();
                let expose = &address_pair[..separator];
                let target = &address_pair[separator + 1..];

                let args = [name, expose.to_string(), target.to_string()];
                Some(ParsedCommand::new(command, args.to_vec()))
            }
            "stop" => {
                let name = input.next().unwrap();

                let args = [name];
                Some(ParsedCommand::new(command, args.to_vec()))
            }
            "slow" => {
                let name = input.next().unwrap();
                let time = input.next().unwrap();

                let args = [name, time];
                Some(ParsedCommand::new(command, args.to_vec()))
            }
            _ => None,
        },
        None => None,
    };

    match result {
        Some(x) => Ok(x),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Cannot parse command"),
        )),
    }
}
