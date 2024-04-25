use std::env;

const ENV_TERMINAL: [&str; 2] = ["TERM", "TERM_PROGRAM"];

pub fn terminal() -> Option<String> {
    for env_term in ENV_TERMINAL {
        if let Ok(term) = env::var(env_term) {
            return Some(term);
        };
    }
    None
}
