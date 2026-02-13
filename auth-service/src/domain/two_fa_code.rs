use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        match code.parse::<usize>() {
            Ok(_) => {
                if code.len() != 6 {
                    return Err("2FA code must be 6 digits".to_owned());
                }

                Ok(TwoFACode(code))
            },
            Err(_) => Err("Code ".to_owned()),
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

        TwoFACode(code)
    }
}


impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}