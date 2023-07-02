use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.unknow2n.co.jp/";
    println!("call {}", url);
    if let Ok(res) = reqwest::get(url).await {
        let body = res.text().await?;
        println!("response is \n{}", body);
    } else {
        println!("error: Webサーバーが見つかりませんでした。");
    }
    // let res = reqwest::get(url).await?;
    // match res.status() {
    //     StatusCode::OK => {
    //         let body = res.text().await?;
    //         println!("response is \n{}", body);
    //     }
    //     StatusCode::NOT_FOUND => {
    //         println!("error: 目的のページがありませんでした。");
    //     }
    //     _ => {
    //         println!("error: その他のエラーが発生しました。");
    //     }
    // }
    Ok(())
}
