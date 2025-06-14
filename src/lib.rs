use std::fmt;

pub struct Client {
    base: String,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            base: "https://hacker-news.firebaseio.com/v0".into(),
        }
    }

    pub fn fetch_top_stories(&self) {
        let _base = &self.base;
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy() -> () {
        print!("All good!");
    }
}
