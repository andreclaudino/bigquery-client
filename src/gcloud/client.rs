use gouth::Builder;
use gouth::Token;
use std::env;

pub struct GCloud {
    token: Token,
}

pub type GCloudFactory = dyn Fn()->GCloud;

impl GCloud {

    pub fn default() -> GCloud {
        let credentials_file = "GOOGLE_APPLICATION_CREDENTIALS";
        GCloud::from_env(credentials_file)
    }

    pub fn from_env(env_name: &str) -> GCloud {
        let credentials_file = env::var(env_name).unwrap();
        GCloud::from_file(credentials_file.as_str())
    }

    pub fn from_file(credentials_file: &str) -> GCloud {
        let token = generate_token(credentials_file);
        GCloud{token}
    }

    pub fn header_value(&self) -> String {
        let header = self.token.header_value().unwrap();
        format!("{}", header)
    }
    
}

pub(crate) trait Endpoint {
    fn endpoint(&self, resource: &str) -> String {
        let base_endpoint = "https://bigquery.googleapis.com";
        format!("{}/{}", base_endpoint, resource)
    }
}

fn generate_token(credentials_file: &str) -> Token {
    Builder::new().file(credentials_file).build().unwrap()
}