use policy_bridge::bridge::PolicyBridge;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut policy_bridge = PolicyBridge::default();
    policy_bridge.load("my_model.onnx")?;
    policy_bridge.info()?;

    policy_bridge.do_something();
    println!("Hello, world!");
    Ok(())
}
