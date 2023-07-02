use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://openccpm.com/redmine/projects.json";
    println!("call {}", url);
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let projects = json["projects"].as_array().unwrap();
    for item in projects {
        let identifier = &item["identifier"].as_str().unwrap();
        let name = &item["name"].as_str().unwrap();
        println!("tag: {} {}", identifier, name);
    }
    Ok(())
}
