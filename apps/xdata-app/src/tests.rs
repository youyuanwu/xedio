use crate::greeter::hello_world::{HelloRequest, greeter_client::GreeterClient};

/// Test calling the gRPC service via NodePort
/// This test assumes the service is deployed in k8s and accessible via NodePort
/// Run with: cargo test -- --ignored
#[tokio::test]
async fn test_call_nodeport() {
    // Replace with your actual node IP
    // You can find it with: kubectl get nodes -o wide
    let node_ip = std::env::var("K8S_NODE_IP").unwrap_or_else(|_| "localhost".to_string());
    let node_port = "30080";
    let endpoint = format!("http://{}:{}", node_ip, node_port);

    println!("Connecting to: {}", endpoint);

    // Create a gRPC client
    let mut client = GreeterClient::connect(endpoint)
        .await
        .expect("Failed to connect to gRPC server");

    // Create a request
    let request = tonic::Request::new(HelloRequest {
        name: "TestClient".to_string(),
    });

    // Call the service
    let response = client
        .say_hello(request)
        .await
        .expect("Failed to call say_hello");

    println!("Response: {:?}", response);

    // Verify the response
    let reply = response.into_inner();
    assert_eq!(reply.message, "Hello TestClient!");
}
