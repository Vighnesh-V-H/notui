use keyring::Entry;

pub struct CredentialManager {
    service: String,
    username: String,
}

impl CredentialManager {
    pub fn new(service: &str, username: &str) -> Self {
        Self {
            service: service.to_string(),
            username: username.to_string(),
        }
    }

    fn entry(&self) -> Result<Entry, keyring::Error> {
        Entry::new(&self.service, &self.username)
    }

    pub fn save_api_key(&self, api_key: &str) -> Result<(), keyring::Error> {
        self.entry()?.set_password(api_key)
    }

    pub fn get_api_key(&self) -> Result<String, keyring::Error> {
        self.entry()?.get_password()
    }

    pub fn delete_api_key(&self) -> Result<(), keyring::Error> {
        self.entry()?.delete_credential()
    }
}
