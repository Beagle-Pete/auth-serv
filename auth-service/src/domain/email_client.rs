use super::Email;
use color_eyre::eyre::Result;

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync {
    async fn send_email(&self, 
        recipient: Email, 
        subject: &str, 
        content: &str
    ) -> Result<()>;
}

impl std::fmt::Debug for dyn EmailClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmailClient")
    }
}