use std::env;
use url::Url;

/**
 * Structure for dealing with environment configuration.
 */
pub struct EnvConfig {
    pub token: String,
    pub mongo_uri: String,
}

impl EnvConfig {
    /**
     * Fetch the environment config.
     */
    pub fn fetch() -> EnvConfig {
        EnvConfig {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            mongo_uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
        }
    }

    /**
     * Fetch the environment config without panicing if env vars are missing.
     */
    pub fn fetch_unsafe() -> EnvConfig {
        EnvConfig {
            token: env::var("TOKEN").unwrap_or(String::from("")),
            mongo_uri: env::var("MONGO_URI").unwrap_or(String::from("")),
        }
    }

    /**
     * Fetch the port of the mongo database.
     */
    pub fn get_mongo_port(&self) -> u16 {
        self.get_mongo_url().port().unwrap_or(27017)
    }

    /**
     * Fetch the address of the mongo database.
     */
    pub fn get_mongo_addr(&self) -> String {
        String::from(
            self.get_mongo_url()
                .host_str()
                .expect("MongoDB URI is invalid"),
        )
    }

    /**
     * Fetch the name of the mongo database.
     */
    pub fn get_mongo_db_name(&self) -> String {
        let url = self.get_mongo_url();
        let mut segments = match url.path_segments() {
            Some(s) => s,
            None => panic!("MongoDB URI is invalid"),
        };

        match segments.next() {
            Some(s) => String::from(s),
            None => panic!("MongoDB URI is invalid"),
        }
    }

    /**
     * Fetch the URL struct representing the Mongo URI.
     */
    fn get_mongo_url(&self) -> Url {
        match Url::parse(&self.mongo_uri[..]) {
            Ok(n) => n,
            Err(e) => panic!("MongoDB URI is invalid: {}", e),
        }
    }
}
