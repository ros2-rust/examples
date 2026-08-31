use anyhow::{Error, Result};
use rclrs::*;

fn main() -> Result<(), Error> {
    let context = Context::default_from_env()?;
    let executor = context.create_basic_executor();

    let node = executor.create_node("minimal_publisher")?;

    let message_type = MessageTypeName {
        package_name: "example_interfaces".to_owned(),
        type_name: "String".to_owned(),
    };

    let publisher = node.create_dynamic_publisher(message_type.clone(), "topic")?;

    let message_metadata = DynamicMessageMetadata::new(message_type)?;

    let mut publish_count: u32 = 1;

    while context.ok() {
        let mut message = message_metadata.create()?;

        let Some(ValueMut::Simple(data)) = message.get_mut("data") else {
            panic!("Unexpected value type, expected Simple value");
        };

        let SimpleValueMut::String(data) = data else {
            panic!("Unexpected value type, expected String");
        };
        *data = format!("Hello, world! {}", publish_count).into();
        println!("Publishing: [{}]", data);
        publisher.publish(message)?;
        publish_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
