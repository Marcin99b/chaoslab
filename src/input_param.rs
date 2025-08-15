use std::env;

#[derive(Debug)]
pub struct InputParam {
    pub name: String,
    pub expose: i32,
    pub target: String,
}

/// example
///
/// my_app_https=1001:127.0.0.1:7176 my_app_http=1002:127.0.0.1:5241
///
/// app_name=expose_port:target_address
pub fn parse_args_from_console() -> Vec<InputParam> {
    let args = env::args();
    parse_args_from_iterator(args)
}

/// example
///
/// my_app_https=1001:127.0.0.1:7176 my_app_http=1002:127.0.0.1:5241
///
/// app_name=expose_port:target_address
pub fn parse_args_from_string(input: String) -> Vec<InputParam> {
    let split = input.split(" ").map(|x| x.to_string());
    parse_args_from_iterator(split)
}

pub fn parse_args_from_iterator(input: impl Iterator<Item = String>) -> Vec<InputParam> {
    input
        .filter_map(|arg| {
            let eq = arg.find('=')?;
            let name = &arg[..eq];
            let mapping = &arg[eq + 1..];
            let colon = mapping.find(':')?;
            let expose = &mapping[..colon];
            let target = &mapping[colon + 1..];
            let expose_port = expose.parse().ok()?;
            Some(InputParam {
                name: name.to_string(),
                expose: expose_port,
                target: target.to_string(),
            })
        })
        .collect()
}
