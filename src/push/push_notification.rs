// FCM Module to send out push notifications
use std::collections::HashMap;
use tokio::prelude::Future;

// Push notification data structure
pub struct PushNotification {
    pub from: String,
    pub to: String,
    pub content: String,
}

impl PushNotification {
    // Create a new Push Notification structure
    pub fn new(from: &String, to: &String, content: &String) -> PushNotification {
        PushNotification {
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
        }
    }

    // Send push notification
    pub fn send(&self) -> Result<(), String> {
        let client = fcm::Client::new().unwrap();

        let mut map = HashMap::new();
        map.insert("from", self.from.to_owned());
        map.insert("content", self.content.to_owned());
        map.insert("message", "Howdy!".to_string());

        let mut builder = fcm::MessageBuilder::new("<FCM API Key>", &self.to);
        if builder.data(&map).is_err() {
            return Err("Builder error".to_string());
        }

        let payload = builder.finalize();

        client
            .send(payload)
            .map(|response| {
                println!("Sent: {:?}", response);
            })
            .map_err(|error| println!("Error: {:?}", error));

        Ok(())
    }
}
