use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

const ADMIN_TOKEN: &str = "test_admin_token";

fn e2e_base_url() -> Option<String> {
    std::env::var("E2E_BASE_URL").ok()
}

struct TestClient {
    client: Client,
    base_url: String,
}

impl TestClient {
    fn new() -> Option<Self> {
        let base_url = e2e_base_url()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", ADMIN_TOKEN).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Some(Self { client, base_url })
    }

    async fn create_place(&self) -> Result<serde_json::Value> {
        let place_data = json!({
            "name": "Test Place",
            "coefficient": 1.5
        });

        let response = self
            .client
            .post(format!("{}/admin/place", self.base_url))
            .json(&place_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_places(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/place", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_place(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/place/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_place(&self, id: &str) -> Result<serde_json::Value> {
        let place_data = json!({
            "name": "Updated Test Place",
            "coefficient": 2.0
        });

        let response = self
            .client
            .put(format!("{}/admin/place/{}", self.base_url, id))
            .json(&place_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_place(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/place/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }

    async fn create_power(&self) -> Result<serde_json::Value> {
        let power_data = json!({
            "min_power": 100,
            "max_power": 150,
            "coefficient": 1.2
        });

        let response = self
            .client
            .post(format!("{}/admin/power", self.base_url))
            .json(&power_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_powers(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/power", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_power(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/power/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_power(&self, id: &str) -> Result<serde_json::Value> {
        let power_data = json!({
            "min_power": 150,
            "max_power": 200,
            "coefficient": 1.4
        });

        let response = self
            .client
            .put(format!("{}/admin/power/{}", self.base_url, id))
            .json(&power_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_power(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/power/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }

    async fn create_kbm(&self) -> Result<serde_json::Value> {
        let kbm_data = json!({
            "class": 3,
            "coefficient": 1.17
        });

        let response = self
            .client
            .post(format!("{}/admin/kbm", self.base_url))
            .json(&kbm_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_kbms(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/kbm", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_kbm(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/kbm/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_kbm(&self, id: &str) -> Result<serde_json::Value> {
        let kbm_data = json!({
            "class": 4,
            "coefficient": 1.0
        });

        let response = self
            .client
            .put(format!("{}/admin/kbm/{}", self.base_url, id))
            .json(&kbm_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_kbm(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/kbm/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }

    async fn create_age_experience(&self) -> Result<serde_json::Value> {
        let age_experience_data = json!({
            "age": 25,
            "experience": 5,
            "coefficient": 1.08
        });

        let response = self
            .client
            .post(format!("{}/admin/age_experience", self.base_url))
            .json(&age_experience_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_age_experiences(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/age_experience", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_age_experience(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/age_experience/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_age_experience(&self, id: &str) -> Result<serde_json::Value> {
        let age_experience_data = json!({
            "age": 30,
            "experience": 8,
            "coefficient": 1.01
        });

        let response = self
            .client
            .put(format!("{}/admin/age_experience/{}", self.base_url, id))
            .json(&age_experience_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_age_experience(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/age_experience/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }

    async fn create_season(&self) -> Result<serde_json::Value> {
        let season_data = json!({
            "months": 6,
            "coefficient": 0.7
        });

        let response = self
            .client
            .post(format!("{}/admin/season", self.base_url))
            .json(&season_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_seasons(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/season", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_season(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/season/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_season(&self, id: &str) -> Result<serde_json::Value> {
        let season_data = json!({
            "months": 9,
            "coefficient": 0.95
        });

        let response = self
            .client
            .put(format!("{}/admin/season/{}", self.base_url, id))
            .json(&season_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_season(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/season/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }

    async fn create_limitation(&self) -> Result<serde_json::Value> {
        let limitation_data = json!({
            "limited": true,
            "coefficient": 1.0
        });

        let response = self
            .client
            .post(format!("{}/admin/limitation", self.base_url))
            .json(&limitation_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_limitations(&self) -> Result<Vec<serde_json::Value>> {
        let response = self
            .client
            .get(format!("{}/admin/limitation", self.base_url))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn get_limitation(&self, id: &str) -> Result<serde_json::Value> {
        let response = self
            .client
            .get(format!("{}/admin/limitation/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn update_limitation(&self, id: &str) -> Result<serde_json::Value> {
        let limitation_data = json!({
            "limited": false,
            "coefficient": 3.16
        });

        let response = self
            .client
            .put(format!("{}/admin/limitation/{}", self.base_url, id))
            .json(&limitation_data)
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(response.json().await?)
    }

    async fn delete_limitation(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!("{}/admin/limitation/{}", self.base_url, id))
            .send()
            .await?;

        assert!(response.status().is_success());
        Ok(())
    }
}

#[tokio::test]
async fn test_place_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_place = client.create_place().await?;
    let place_id = created_place["_id"]["$oid"]
        .as_str()
        .expect("Place ID should be present");

    let places = client.get_places().await?;
    assert!(!places.is_empty());

    let place = client.get_place(place_id).await?;
    assert_eq!(place["name"], "Test Place");

    let updated_place = client.update_place(place_id).await?;
    assert_eq!(updated_place["name"], "Updated Test Place");

    client.delete_place(place_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_power_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_power = client.create_power().await?;
    let power_id = created_power["_id"]["$oid"]
        .as_str()
        .expect("Power ID should be present");

    let powers = client.get_powers().await?;
    assert!(!powers.is_empty());

    let power = client.get_power(power_id).await?;
    assert_eq!(power["min_power"], 100);

    let updated_power = client.update_power(power_id).await?;
    assert_eq!(updated_power["min_power"], 150);

    client.delete_power(power_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_kbm_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_kbm = client.create_kbm().await?;
    let kbm_id = created_kbm["_id"]["$oid"]
        .as_str()
        .expect("KBM ID should be present");

    let kbms = client.get_kbms().await?;
    assert!(!kbms.is_empty());

    let kbm = client.get_kbm(kbm_id).await?;
    assert_eq!(kbm["class"], 3);

    let updated_kbm = client.update_kbm(kbm_id).await?;
    assert_eq!(updated_kbm["class"], 4);

    client.delete_kbm(kbm_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_age_experience_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_age_experience = client.create_age_experience().await?;
    let age_experience_id = created_age_experience["_id"]["$oid"]
        .as_str()
        .expect("AgeExperience ID should be present");

    let age_experiences = client.get_age_experiences().await?;
    assert!(!age_experiences.is_empty());

    let age_experience = client.get_age_experience(age_experience_id).await?;
    assert_eq!(age_experience["age"], 25);

    let updated_age_experience = client.update_age_experience(age_experience_id).await?;
    assert_eq!(updated_age_experience["age"], 30);

    client.delete_age_experience(age_experience_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_season_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_season = client.create_season().await?;
    let season_id = created_season["_id"]["$oid"]
        .as_str()
        .expect("Season ID should be present");

    let seasons = client.get_seasons().await?;
    assert!(!seasons.is_empty());

    let season = client.get_season(season_id).await?;
    assert_eq!(season["months"], 6);

    let updated_season = client.update_season(season_id).await?;
    assert_eq!(updated_season["months"], 9);

    client.delete_season(season_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_limitation_crud() -> Result<()> {
    let Some(client) = TestClient::new() else {
        return Ok(());
    };

    let created_limitation = client.create_limitation().await?;
    let limitation_id = created_limitation["_id"]["$oid"]
        .as_str()
        .expect("Limitation ID should be present");

    let limitations = client.get_limitations().await?;
    assert!(!limitations.is_empty());

    let limitation = client.get_limitation(limitation_id).await?;
    assert_eq!(limitation["limited"], true);

    let updated_limitation = client.update_limitation(limitation_id).await?;
    assert_eq!(updated_limitation["limited"], false);

    client.delete_limitation(limitation_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_unauthorized_access() -> Result<()> {
    let Some(base_url) = e2e_base_url() else {
        return Ok(());
    };

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let response = client
        .get(format!("{}/admin/place", base_url))
        .send()
        .await?;

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_invalid_token() -> Result<()> {
    let Some(base_url) = e2e_base_url() else {
        return Ok(());
    };

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let response = client
        .get(format!("{}/admin/place", base_url))
        .send()
        .await?;

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

    Ok(())
}
