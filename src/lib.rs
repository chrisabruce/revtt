#[derive(Debug)]
pub struct Connection {
    username: String,
    password: String,
}

impl Connection {
    pub fn new(username: &str, password: &str) -> Self {
        Connection {
            username: username.to_string(),
            password: password.to_string(),
        }
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
