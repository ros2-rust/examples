use rclrs::*;
use anyhow::{Result, Error};
use example_interfaces::action::{Fibonacci, Fibonacci_Goal};
use std::time::Duration;
use futures::StreamExt;

fn main() -> Result<(), Error> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();

    let node = executor.create_node("action_client")?;
    log_info!(node.logger(), "Action client node starting ...");

    let client = node
        .create_action_client::<Fibonacci>(&"action_name")
        .unwrap();

    log_info!(node.logger(), "Wait for action server ..."); // Completly arbitrary
    std::thread::sleep(Duration::from_secs(1));

    let request = client.request_goal(Fibonacci_Goal { order: 10 });

    let promise = executor.commands().run(async move {
        let mut goal_client_stream = request.await.unwrap().stream();
        while let Some(event) = goal_client_stream.next().await {
            match event {
                GoalEvent::Feedback(feedback) => {
                    log_info!(node.logger(), "Received feedback: {}", feedback.sequence.last().unwrap());
                }
                GoalEvent::Result((status, result)) => {
                    match status {
                        GoalStatusCode::Succeeded =>{log_info!(node.logger(), "Goal succeeded, complete result {:?}", result.sequence)}
                        GoalStatusCode::Cancelled =>{log_info!(node.logger(), "Goal canceled before end, result {:?}", result.sequence)}
                        _ => {},
                    }
                    
                    return;
                }
                _ => {},
            }
        }
    });

    executor.spin(SpinOptions::default().until_promise_resolved(promise));
    Ok(())
}