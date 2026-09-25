use anyhow::{Error, Result};
use rclrs::*;
use ros_env::example_interfaces::srv::*;

fn handle_service(request: AddTwoInts_Request, info: ServiceInfo) -> AddTwoInts_Response {
    let timestamp = info
        .received_timestamp
        .map(|t| format!(" at [{t:?}]"))
        .unwrap_or(String::new());

    println!("request{timestamp}: {} + {}", request.a, request.b);
    AddTwoInts_Response {
        sum: request.a + request.b,
    }
}

fn main() -> Result<(), Error> {
    let mut executor = Context::default_from_env()?.create_basic_executor();

    let node = executor.create_node("minimal_service")?;

    let server = node.create_service::<AddTwoInts, _>("add_two_ints", handle_service)?;

    // Enable introspection for this service
    // if you want to be able to introspect the service calls e.g. with ros2 service echo
    server.configure_introspection(ServiceIntrospectionState::Contents)?;

    println!("Starting server");
    executor.spin(SpinOptions::default()).first_error()?;
    Ok(())
}
