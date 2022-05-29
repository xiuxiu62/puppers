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
}
