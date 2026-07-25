use anyhow::{anyhow, Context, Result};
use mongodb::{bson::doc, options::ClientOptions, Client};
use regex::Regex;
use std::{fs, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    // Настройки
    let html_path = "data/place.html";
    let mongo_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    // Чтение HTML
    let html = fs::read_to_string(Path::new(html_path))
        .with_context(|| format!("failed to read {}", html_path))?;

    // Подключение к Mongo
    let client_opts = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_opts)?;
    let db = client.database("openapi");
    let collection = db.collection::<mongodb::bson::Document>("places");

    // Регексы для парсинга TR и TD
    let re_tr = Regex::new(r"(?is)<tr>(.*?)</tr>").unwrap();
    let re_td = Regex::new(r"(?is)<td[^>]*>(.*?)</td>").unwrap();

    let mut inserted = 0usize;
    let mut updated = 0usize;
    let mut skipped = 0usize;

    for caps in re_tr.captures_iter(&html) {
        let tr = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        // Извлекаем все TD в строке
        let tds: Vec<String> = re_td
            .captures_iter(tr)
            .filter_map(|c| c.get(1))
            .map(|m| html_unescape_and_trim(m.as_str()))
            .collect();

        // Ожидаем минимум 3 колонки: [№, Регион, Коэффициент ТС, ...]
        if tds.len() < 3 {
            skipped += 1;
            continue;
        }

        let name = tds[1].trim();
        let coef_raw = tds[2].trim();

        // Пропуск пустых/coefficient отсутствует
        if name.is_empty() || coef_raw.is_empty() {
            skipped += 1;
            continue;
        }

        // Коэффициент в данных записан с запятой
        let coef_str = coef_raw.replace(',', ".");
        let coefficient: f64 = match coef_str.parse() {
            Ok(v) => v,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };

        // Upsert по name
        let filter = doc! { "name": name };
        let update = doc! {
            "$set": { "name": name, "coefficient": coefficient },
            "$setOnInsert": { }
        };
        let opts = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        let res = collection
            .update_one(filter, update)
            .with_options(opts)
            .await
            .map_err(|e| anyhow!("mongo update failed for '{}': {}", name, e))?;

        if res.matched_count == 0 {
            inserted += 1;
        } else {
            updated += 1;
        }
    }

    println!(
        "Seed places done: inserted={}, updated={}, skipped={}",
        inserted, updated, skipped
    );
    Ok(())
}

fn html_unescape_and_trim(s: &str) -> String {
    // Убираем теги <p> и прочее простым способом и декодируем основные HTML-сущности
    let no_tags = Regex::new(r"(?is)<[^>]+>").unwrap().replace_all(s, "");
    let s = no_tags.trim();
    htmlescape::decode_html(s).unwrap_or_else(|_| s.to_string())
}
