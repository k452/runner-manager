use aws_sdk_sqs::types::Message;

pub fn extract_job_id_from_sqs_message(message: &Message) -> String {
    let body = message.body.as_ref().unwrap();
    body.to_string()
}
