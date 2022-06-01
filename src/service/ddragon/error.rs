#[derive(Clone, Debug)]
enum DDragonError {
    LeagueProcessNotFound,
    UnsupportedPlatform,
}

impl std::fmt::Display for DDragonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            DDragonError::LeagueProcessNotFound => "Ensure a League client process exists",
            DDragonError::UnsupportedPlatform => "Only macOS and Windows are supported",
        };
        write!(f, "{}", message)
    }
}
