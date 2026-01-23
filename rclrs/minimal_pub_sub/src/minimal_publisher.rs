use anyhow::{Error, Result};
use rclrs::*;

use clap::Parser;

/// Simple program to publish ROS 2 messages using clap
#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Number of messages to publish
    #[arg(short, long)]
    count: Option<u32>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse_from(extract_non_ros_args(std::env::args())?);

    let context = Context::default_from_env()?;
    let executor = context.create_basic_executor();

    let node = executor.create_node("minimal_publisher")?;

    let publisher = node.create_publisher::<example_interfaces::msg::String>("topic")?;

    let mut message = example_interfaces::msg::String::default();

    let mut publish_count: u32 = 1;

    while context.ok() && args.count.map_or(true, |count| publish_count <= count) {
        message.data = format!("Hello, world! {}", publish_count);
        println!("Publishing: [{}]", message.data);
        publisher.publish(&message)?;
        publish_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
