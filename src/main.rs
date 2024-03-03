use aws_sdk_sqs::{Client, Error};
use dotenvy::dotenv;
use std::env;
pub mod container;
pub mod utils;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    dotenv().expect(".env file not found");
    let queue_url = env::var("SQS_ENDPOINT").expect("no such var.");

    loop {
        println!("{}", "取得開始");
        let receive_messages = client
            .receive_message()
            .queue_url(&queue_url)
            .wait_time_seconds(20)
            .send()
            .await?;

        for message in receive_messages.messages.unwrap_or_default() {
            container::run(utils::extract_job_id_from_sqs_message(&message));

            client
                .delete_message()
                .queue_url(&queue_url)
                .set_receipt_handle(message.receipt_handle);
        }

        println!("{}", "取得完了");
    }
}
