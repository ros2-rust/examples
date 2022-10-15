use std::sync::{Arc, Mutex};
use std::time::Duration;

use std_msgs::msg::String as StringMsg;

struct RepublisherNode {
    node: rclrs::Node,
    _subscription: Arc<rclrs::Subscription<StringMsg>>,
    publisher: rclrs::Publisher<StringMsg>,
    data: Arc<Mutex<Option<StringMsg>>>,
}

impl RepublisherNode {
    fn new(context: &rclrs::Context) -> Result<Self, rclrs::RclrsError> {
        let mut node = rclrs::Node::new(context, "republisher")?;
        let data = Arc::new(Mutex::new(None));
        let _subscription = {
            let data = Arc::clone(&data);
            node.create_subscription(
                "in_topic",
                rclrs::QOS_PROFILE_DEFAULT,
                move |msg: StringMsg| {
                    *data.lock().unwrap() = Some(msg);
                },
            )?
        };
        let publisher = node.create_publisher("out_topic", rclrs::QOS_PROFILE_DEFAULT)?;
        Ok(Self {
            node,
            _subscription,
            publisher,
            data,
        })
    }

    fn republish(&self) -> Result<(), rclrs::RclrsError> {
        if let Some(s) = &*self.data.lock().unwrap() {
            self.publisher.publish(s)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let republisher = Arc::new(RepublisherNode::new(&context)?);
    let republisher_other_thread = Arc::clone(&republisher);
    std::thread::spawn(move || -> Result<(), rclrs::RclrsError> {
        loop {
            std::thread::sleep(Duration::from_millis(1000));
            republisher_other_thread.republish()?;
        }
    });
    rclrs::spin(&republisher.node)
}
