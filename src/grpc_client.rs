use tonic::transport::Channel;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio::io::{self, AsyncBufReadExt};

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
    chat_service_client::ChatServiceClient,
    PaymentRequest, TransactionRequest, ChatMessage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Testing Payment Service ---");
    let mut payment_client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    
    let payment_request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let payment_response = payment_client.process_payment(payment_request).await?;
    println!("PAYMENT RESPONSE = {:?}\n", payment_response.into_inner());


    println!("--- Testing Transaction Service ---");
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    
    let transaction_request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut transaction_stream = transaction_client.get_transaction_history(transaction_request).await?.into_inner();
    
    while let Some(transaction) = transaction_stream.message().await? {
        println!("Transaction: {:?}", transaction);
    }
    println!("\n");


    println!("--- Testing Chat Service ---");
    println!("Silakan ketik pesan Anda di bawah ini dan tekan Enter:");
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut chat_client = ChatServiceClient::new(channel);

    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                continue;
            }

            let message = ChatMessage {
                user_id: "user_123".to_string(),
                message: line,
            };

            if tx.send(message).await.is_err() {
                eprintln!("Failed to send message to server.");
                break;
            }
        }
    });

    let request = tonic::Request::new(ReceiverStream::new(rx));
    let mut response_stream = chat_client.chat(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("Server says: {:?}", response);
    }

    Ok(())
}