use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, TimeZone};
use scraper::{Html, Selector};
enum Stock {
    Many,
    Countable(u64),
}

#[derive(Debug, Default)]
struct Product {
    name: String,
    code: String,
    sku_num: String,
    postage: u64,
    price: u64,
    stock: Option<u64>,
    next_arrival: Option<String>,
    url: String,
    date: String,
}

impl Product {
    fn new() -> Self {
        Default::default()
    }
}

fn scrape(html: &str) -> Result<Product> {
    let product = Html::parse_document(html)
        .select(&Selector::parse("div.data tbody").unwrap())
        .next()
        .ok_or_else(|| anyhow!("failed to parse html"))?
        .select(&Selector::parse("tr").unwrap())
        .fold(Product::new(), |mut product, tr| {
            // データ
            let td = tr.select(&Selector::parse("td").unwrap()).next().unwrap();
            if let Some(th) = tr.select(&Selector::parse("th").unwrap()).next() {
                match th.text().next().unwrap_or("").trim() {
                    "名前" => product.name = td.text().next().unwrap().to_string(),
                    "コード番号" => product.code = td.text().next().unwrap().to_string(),
                    "SKU#" => product.sku_num = td.text().next().unwrap().to_string(),
                    "送料区分" => {
                        product.postage = td
                            .select(&Selector::parse("span").unwrap())
                            .next()
                            .unwrap()
                            .text()
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap();
                    }
                    "税込単価" => {
                        product.price = td
                            .select(&Selector::parse("span.price").unwrap())
                            .next()
                            .unwrap()
                            .text()
                            .next()
                            .unwrap()
                            .trim()
                            .replace(",", "")
                            .parse()
                            .unwrap();
                    }
                    "在庫" => {}
                    "次回入荷" => {}
                    "短縮URL" => {
                        product.url = td
                            .select(&Selector::parse("a").unwrap())
                            .next()
                            .unwrap()
                            .value()
                            .attr("href")
                            .unwrap()
                            .trim()
                            .to_string();
                    }
                    "公開日" => {
                        product.date = "".to_string();
                        let a = td.text().next().unwrap().trim().replace("以前", "1日");
                        product.date = a;
                    }
                    _ => {}
                }
            }

            product
        });

    Ok(product)
}

#[tokio::main]
async fn main() -> Result<()> {
    // arduino uno r3
    let url = "https://www.switch-science.com/catalog/3/";
    let html = reqwest::get(url).await?.text().await?;

    let product = scrape(&html)?;
    println!("{:?}", product);
    Ok(())
}
