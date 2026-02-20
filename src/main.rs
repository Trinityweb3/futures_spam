use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut url: String = String::from("wss://fstream.binance.com/ws/");
    let url_end: String = String::from("usdt@trade");

    println!("введите тикер для Binance futures латиницей с маленькой буквы. Пример - sol");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let ticker: &str = &input.trim();

    url.push_str(ticker);
    url.push_str(&url_end);
    println!("{}", url);

    let (ws_stream, _) = match connect_async(url).await {
        Ok(res) => res,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    println!("Connected to binance futures");

    let (_, mut read) = ws_stream.split();

    loop {
        match read.next().await {
            Some(Ok(msg)) => {
                match msg.to_text() {
                    Ok(text) => {
                        match serde_json::from_str::<Value>(text) {
                            Ok(json) => {
                                match json.get("p") {
                                    Some(price) => {
                                        println!("{}/USDT price: {}", ticker.to_string().to_uppercase(), price);
                                    }
                                    None => {}
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Some(Err(e)) => {
                println!("WebSocket error: {}", e);
                break;
            }
            None => break
        }    
    }
}

