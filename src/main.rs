extern crate reqwest;

async fn get_month_stats() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://192.168.1.1/api/monitoring/month_statistics")
    .await?
    .text()
    .await?;

    return Ok(body)
}

#[tokio::main]
async fn main() {
    let body = get_month_stats()
        .await
        .unwrap();

    let doc = roxmltree::Document::parse(&body)
        .unwrap();

    let current_month_download: u64 = doc.descendants()
                                    .find(
                                        |node| node.has_tag_name("CurrentMonthDownload"))
                                        .unwrap()
                                        .text()
                                        .unwrap()
                                        .parse::<u64>()
                                        .unwrap();
                                        
    let current_month_download_in_gio: f32 = current_month_download as f32 / 1024_f32.powi(3);                                 
    let total_month_download_in_gio: f32 = 200.0;
    let available_month_download_in_gio: f32 = total_month_download_in_gio - current_month_download_in_gio;
    let consumed_ratio_in_percent: f32 = 100.0 * current_month_download_in_gio/total_month_download_in_gio;

    println!("Consumed: {:.2}Gio\tAvailable: {:.2}Gio\tPercentage: {:.1}%", current_month_download_in_gio, available_month_download_in_gio, consumed_ratio_in_percent);
}

