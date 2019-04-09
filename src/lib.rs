use cookie::{Cookie, CookieJar};
use reqwest::Client;

pub use reqwest::Error;

const BASE_URL: &str = "https://revolutiontt.me";
const HEADER_SET_COOKIE: &str = "Set-Cookie";

#[derive(Debug)]
pub struct Connection {
    client: Client,
    jar: CookieJar,
    username: String,
    password: String,
}

impl Connection {
    pub fn new(username: &str, password: &str) -> Self {
        Connection {
            client: reqwest::Client::new(),
            jar: CookieJar::new(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Returns `Error` if login fails, otherwise `Ok()`
    pub fn login(&mut self) -> Result<(), Error> {
        let params = [
            ("username", &self.username),
            ("password", &self.password),
            ("submit", &"login".to_string()),
        ];

        let mut res = self
            .client
            .post(&format!("{}/login.php", BASE_URL))
            .form(&params)
            .send()?;
        self.store_cookies(&res);

        println!("{:?}", res.text());
        Ok(())
    }

    fn store_cookies(&mut self, res: &reqwest::Response) {
        res.headers()
            .get_all(HEADER_SET_COOKIE)
            .into_iter()
            .map(|header_value| header_value.to_str())
            .filter_map(Result::ok)
            .map(|s| s.to_string())
            .for_each(|cookie_string| {
                if let Ok(cookie) = Cookie::parse(cookie_string) {
                    self.jar.add_original(cookie);
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let r = Connection::new(&"test", &"pass");
        assert_eq!(r.username, "test");
        assert_eq!(r.password, "pass");
    }
}
