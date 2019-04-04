use reqwest::Client;

pub use reqwest::Error;

const BASE_URL: &str = "https://revolutiontt.me";

#[derive(Debug)]
pub struct Connection {
    client: Client,
    username: String,
    password: String,
}

impl Connection {
    pub fn new(username: &str, password: &str) -> Self {
        Connection {
            client: reqwest::Client::new(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Returns `Error` if login fails, otherwise `Ok()`
    pub fn login(&self) -> Result<(), Error> {
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

        println!("{:?}", res.text());
        Ok(())
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
