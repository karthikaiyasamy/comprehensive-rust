use tokio::sync::broadcast;

pub struct ChatHub {
    tx: broadcast::Sender<String>,
}

impl ChatHub {
    pub fn new(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        ChatHub { tx }
    }

    pub fn broadcast(&self, message: String) -> Result<usize, broadcast::error::SendError<String>> {
        self.tx.send(message)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_broadcast() {
        let hub = ChatHub::new(16);
        let mut sub1 = hub.subscribe();
        let mut sub2 = hub.subscribe();

        hub.broadcast("Hello Rustacean!".to_string()).unwrap();

        assert_eq!(sub1.recv().await.unwrap(), "Hello Rustacean!");
        assert_eq!(sub2.recv().await.unwrap(), "Hello Rustacean!");
    }
}
