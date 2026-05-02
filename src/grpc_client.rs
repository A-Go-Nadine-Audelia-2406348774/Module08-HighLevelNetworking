pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_client::PaymentServiceClient,
    transaction_service_client::TransactionServiceClient,
    PaymentRequest, TransactionRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Testing Payment Service ---");
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;
    
    let payment_request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let payment_response = client.process_payment(payment_request).await?;
    println!("PAYMENT RESPONSE = {:?}\n", payment_response.into_inner());

    println!("--- Testing Transaction Service (Streaming) ---");
    let mut transaction_client = TransactionServiceClient::connect("http://[::1]:50051").await?;
    
    let transaction_request = tonic::Request::new(TransactionRequest {
        user_id: "user_123".to_string(),
    });

    let mut stream = transaction_client.get_transaction_history(transaction_request).await?.into_inner();
    
    while let Some(transaction) = stream.message().await? {
        println!("Stream Received: {:?}", transaction);
    }

    Ok(())
}