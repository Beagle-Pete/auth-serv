use color_eyre::eyre::{Result, eyre};
use secrecy::{ExposeSecret, SecretString};

use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct TwoFACode(SecretString);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self> {
        match code.parse::<usize>() {
            Ok(_) => {
                if code.len() != 6 {
                    return Err(eyre!("Invalid 2FA code"));
                }

                Ok(TwoFACode(SecretString::new(code.into_boxed_str())))
            },
            Err(_) => Err(eyre!("Invalid 2FA code")),
        }        
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut code = rand::rng().random_range(0..=999_999).to_string();
        
        if code.len() < 6 {
            let leading_zeros = "0".repeat(6-code.len());
            code = format!("{}{}", leading_zeros, code);
        } 

        TwoFACode(SecretString::new(code.into_boxed_str()))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}