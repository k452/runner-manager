use anyhow::{Context, Ok, Result};
use aws_sdk_sqs::types::Message;
use aws_sdk_sqs::Client;

use crate::utils::CONFIG;

pub struct Sqs {
    client: Client,
}

pub fn new(client: Client) -> Sqs {
    Sqs { client }
}

impl Sqs {
    pub async fn receive(&self) -> Result<Vec<Message>> {
        let receive_messages = self
            .client
            .receive_message()
            .queue_url(format!("{0}", CONFIG.sqs_endpoint))
            .wait_time_seconds(20)
            .send()
            .await?
            .messages
            .context("")?;

        Ok(receive_messages)
    }

    pub async fn delete(&self, receipt_handle: Option<String>) -> Result<()> {
        let _ = self
            .client
            .delete_message()
            .queue_url(format!("{0}", CONFIG.sqs_endpoint))
            .set_receipt_handle(receipt_handle)
            .send()
            .await;

        Ok(())
    }
}
