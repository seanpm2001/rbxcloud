use rbxcloud::rbx::{PublishVersionType, RbxCloud};

#[tokio::main]
async fn main() {
    // Inputs:
    let api_key = String::from("MY_API_KEY");
    let universe_id = 9876543210;
    let place_id = 1234567890;
    let filename = "my_experience.rbxl";
    let publish_version_type = PublishVersionType::Published;

    // Define RbxCloud instance:
    let cloud = RbxCloud::new(api_key, universe_id);
    let experience = cloud.experience(place_id);

    // Publish place:
    let publish_result = experience.publish(filename, publish_version_type).await;
    match publish_result {
        Ok(result) => {
            println!("Published place! New version: {}", result.version_number);
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}
