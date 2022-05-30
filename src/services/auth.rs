use std::collections::HashMap;

use super::{ServiceError, ServiceResult};
use sysinfo::{System, SystemExt};

#[derive(Debug, Default)]
struct Auth;

impl Auth {
    pub fn init(&self) -> ServiceResult<()> {
        let process_name = if cfg!(windows) {
            "LeagueClientUx.exe"
        } else if cfg!(windows) {
            "LeagueClientUx"
        } else {
            return Err(ServiceError::UnsupportedPlatform);
        };

        let system = System::new_all();
        let _temp = system.processes_by_exact_name(process_name).into_iter();

        Ok(())
    }

    fn parse_commandline_args<'a>(args: &'a [&str]) -> HashMap<String, String> {
        let filter_map_arg = |arg: &'a str| -> Option<Vec<&'a str>> {
            if !arg.contains('=') {
                return None;
            }

            let args: Vec<&str> = match arg.strip_prefix("-") {
                Some(arg) => arg.split('=').map(|segment| segment).collect(),
                None => return None,
            };

            match args.len() {
                2 => Some(args),
                _ => None,
            }
        };

        let fold_args =
            |mut acc: HashMap<String, String>, segments: Vec<&str>| -> HashMap<String, String> {
                acc.insert(segments[0].to_owned(), segments[1].to_owned());
                acc
            };

        args.into_iter()
            .map(|arg| arg.to_owned())
            .filter_map(filter_map_arg)
            .fold(HashMap::new(), fold_args)
    }
}
