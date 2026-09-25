use rclrs::*;
use ros_env::*;
use std::sync::Arc;

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("parameter_demo")?;

    let greeting: MandatoryParameter<Arc<str>> = node
        .declare_parameter("greeting")
        .default("Hello".into())
        .mandatory()?;

    let reliability_override = node
        .declare_parameter::<Arc<str>>("qos_override/reliability")
        .optional()?
        .get();

    // Use PrimitiveOptions and override the reliability if needed.
    // PrimitiveOptions ensures that the subscription will use the default
    // QoS of a subscription for any setting that is not overridden.
    let mut subscription_options = PrimitiveOptions::new("greet");
    if let Some(reliability_override_str) = reliability_override {
        match &*reliability_override_str {
            "reliable" => {
                subscription_options.reliability = Some(QoSReliabilityPolicy::Reliable);
            }
            "best_effort" => {
                subscription_options.reliability = Some(QoSReliabilityPolicy::BestEffort);
            }
            "best_available" => {
                subscription_options.reliability = Some(QoSReliabilityPolicy::BestAvailable);
            }
            "system_default" => {
                subscription_options.reliability = Some(QoSReliabilityPolicy::SystemDefault);
            }
            x => {
                panic!("Unknown reliability override string: {x}");
            }
        }
    }

    let _subscription = node.create_subscription(
        subscription_options,
        move |msg: example_interfaces::msg::String| {
            println!("{}, {}", greeting.get(), msg.data);
        },
    )?;

    println!(
        "Ready to provide a greeting. \
        \n\nTo see a greeting, try running\n \
        $ ros2 topic pub greet example_interfaces/msg/String \"data: Alice\"\
        \n\nTo change the kind of greeting, try running\n \
        $ ros2 param set parameter_demo greeting \"Guten tag\"\n"
    );
    executor.spin(SpinOptions::default()).first_error()
}
