use std::env;

pub struct Config {
    pub server: Server,
    pub user: UserInfo,
}

pub struct Server {
    pub host: String,
    pub port: String,
}

pub struct UserInfo {
    pub nick: String,
    pub user: String,
    pub password: Option<String>,
}

impl Config {
    pub fn read_config() -> Config {
        let password: Option<String> = match env::var("ISLABOT_USER_PASS") {
            Ok(pass) => Some(pass),
            Err(e) => None,
        };
        let server = Server {
            host: env::var("ISLABOT_SERVER_HOST").unwrap(),
            port: env::var("ISLABOT_SERVER_PORT").unwrap(),
        };

        let user_info = UserInfo {
            nick: env::var("ISLABOT_USER_NICK").unwrap(),
            user: env::var("ISLABOT_USER_INFO").unwrap(),
            password: password,
        };

        Config {
            server: server,
            user: user_info,
        }
    }
}
