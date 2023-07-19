use std::env;

type Host = String;
type Port = u16;

pub struct Config {
    pub host: Host,
    pub port: Port,
}

pub fn config() -> Config {
    Config {
        host: env::var("HOST").unwrap_or(String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or("8081".to_string())
            .parse::<u16>()
            .unwrap(),
    }
}
