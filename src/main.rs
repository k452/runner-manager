use aws_sdk_sqs::Client;

pub mod container;
pub mod github;
pub mod sqs;
pub mod utils;

#[tokio::main]
pub async fn main() {
    let sqs = sqs::new(Client::new(&aws_config::load_from_env().await));

    loop {
        let receive_messages = match sqs.receive().await {
            Ok(v) => v,
            Err(_err) => {
                // eprintln!("receive failed.: {}", err);
                continue;
            }
        };

        for message in receive_messages {
            let job_id = match utils::extract_job_id_from_sqs_message(&message) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("extract_job_id_from_sqs_message failed.: {}", err);
                    continue;
                }
            };

            let generated_jit_config = match github::generate_jit_config(&job_id).await {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("generate_jit_config failed.: {}", err);
                    continue;
                }
            };

            container::run(&job_id, &generated_jit_config);

            match sqs.delete(message.receipt_handle).await {
                Err(err) => {
                    eprintln!("delete failed.: {}", err);
                    continue;
                }
                _ => continue,
            };
        }
    }
}
