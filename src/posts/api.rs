use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

pub async fn get_post(id: u32) -> Result<Post, Box<dyn std::error::Error>> {
    let url = format!("{}/posts/{}", BASE_URL, id);
    let response = reqwest::get(&url).await?.json::<Post>().await?;
    Ok(response)
}

pub async fn get_posts() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let url = format!("{}/posts", BASE_URL);
    let response = reqwest::get(url).await?.json::<Vec<Post>>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_post() {
        // Arrange
        let id = 1;
        // Act
        let result = get_post(id).await;
        // Assert
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.id, id);
    }
    #[tokio::test]
    async fn test_get_post_empty() {
        // Arrange
        let id = 10000;
        // Act
        let result = get_post(id).await;
        // Assert
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_get_posts() {
        // Act
        let result = get_posts().await;
        // Assert
        assert!(result.is_ok());
        let posts = result.unwrap();
        assert!(!posts.is_empty());
        assert_eq!(posts.len(), 100);
    }
}
