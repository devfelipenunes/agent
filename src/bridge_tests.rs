#[cfg(test)]
mod tests {
    use crate::bridge::{PythonBridge, BridgeEvent};
    use tokio::sync::mpsc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_bridge_lifecycle() {
        let (tx, mut rx) = mpsc::channel(100);
        let mut bridge = PythonBridge::start(tx).await.expect("Failed to start bridge");

        // 1. Expect System Start
        let mut system_started = false;
        while let Ok(Some(ev)) = tokio::time::timeout(Duration::from_secs(5), rx.recv()).await {
            if ev.r#type == "system" && ev.message.contains("Started") {
                system_started = true;
                break;
            }
        }
        assert!(system_started, "System did not start");

        // 2. Send Query
        bridge.send_query("oi").await.expect("Failed to send query");

        // 3. Expect Acknowledgment and PM Response
        let mut received_ack = false;
        let mut received_start = false;
        let mut received_token = false;

        let start_time = std::time::Instant::now();
        while start_time.elapsed() < Duration::from_secs(15) {
            if let Ok(Some(ev)) = tokio::time::timeout(Duration::from_millis(500), rx.recv()).await {
                if ev.message.contains("Recebido") { received_ack = true; }
                if ev.r#type == "message_start" && ev.agent == "Aether-PM" { received_start = true; }
                if ev.r#type == "token" { received_token = true; }
                
                if received_ack && received_start && received_token { break; }
            }
        }

        assert!(received_ack, "Did not receive 'Recebido' acknowledgment");
        assert!(received_start, "Did not receive 'message_start' from PM");
        assert!(received_token, "Did not receive any tokens");
    }
}
