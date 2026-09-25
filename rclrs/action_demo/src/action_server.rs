use rclrs::*;
use tokio::sync::mpsc::unbounded_channel;
use std::time::Duration;
use anyhow::{Error, Result};
use example_interfaces::action::{Fibonacci, Fibonacci_Feedback, Fibonacci_Result};

async fn fibonacci_action(node: Node, handle: RequestedGoal<Fibonacci>) -> TerminatedGoal {
    let goal_order = handle.goal().order; // Get the fibonacci order inside the requested goal
    log_info!(node.logger(), "Received goal with order: {} from client", goal_order);

    // Reject the forbidden goal
    if goal_order < 0 {
        log_error!(node.logger(), "Rejecting goal, can't compute Fibonacci sequence of negative number");
        return handle.reject(); 
    }

    let mut result = Fibonacci_Result::default(); // Initialize the result variable
    let mut sequence = Vec::new(); // Initialize the feedback

    // Notifying the acceptance of the goal and starting the execution phase
    let executing = match handle.accept().begin() {
        BeginAcceptedGoal::Execute(executing) => executing,
        BeginAcceptedGoal::Cancel(cancelling) => {
            return cancelling.cancelled_with(result);
        }
    };

    let (sender, mut receiver) = unbounded_channel();

    // Execution thread computing the actual Fibonacci sequence
    std::thread::spawn(move || {
        let mut previous = 0;
        let mut current = 1;

        for _ in 0..goal_order {
            if let Err(_) = sender.send(current) {
                return;
            }

            let next = previous + current;
            previous = current;
            current = next;
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    // Consuming the successive results comming from the execution thread
    loop {
        match executing.unless_cancel_requested(receiver.recv()).await {
            Ok(Some(next)) => {
                // Still executing, push the current result as feedback
                sequence.push(next);
                executing.publish_feedback(Fibonacci_Feedback {
                    sequence: sequence.clone(),
                });
            }
            Ok(None) => {
                // The end of the sequence is reached, return result
                log_info!(node.logger(), "Sequence end reached, action succeeded");
                result.sequence = sequence;
                return executing.succeeded_with(result);
            }
            Err(_) => {
                // Cancel received, end the current execution
                log_warn!(node.logger(), "Goal cancelled");
                let cancelling = executing.begin_cancelling();
                result.sequence = sequence;
                return cancelling.cancelled_with(result);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();

    let node = executor.create_node("action_server")?;
    log_info!(node.logger(), "Action server node starting ...");

    let action_node = node.clone();
    let _action = node.create_action_server(
    &"action_name",
    move |handle| {
        fibonacci_action(action_node.clone(), handle)
    }).unwrap();

    executor.spin(SpinOptions::default()).first_error()?;
    Ok(())
}
