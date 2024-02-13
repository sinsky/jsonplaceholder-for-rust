use serde::Deserialize;

/// jsonplaceholderのpost構造
#[derive(Debug, Deserialize)]

pub struct Post {
    /// * `u32` - Post項目のID
    #[serde(rename = "userId")]
    pub user_id: u32,
    /// * `u32` - Post項目のID
    pub id: u32,
    /// * `String` - Post項目のタイトル
    pub title: String,
    /// * `String` - Post項目の本文
    pub body: String,
}

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

/// 指定されたIDを持つPost項目を取得します。
/// # 引数
/// * `id` - Post項目のID
/// # 戻り値
/// * `Result<Post, Box<dyn std::error::Error>>` - Post項目を返します。
/// * `Box<dyn std::error::Error>` - エラーを返します。
/// # 例外
/// * `std::error::Error` - エラーを返します。
/// # 詳細
/// この関数は、指定されたIDを持つPost項目を取得します。
/// ## 例
/// ```rust
/// use posts::api::get_post;
/// async {
///     let post = get_post(1).await.unwrap();
///     assert_eq!(post.id, 1);
/// };
/// ```
/// # 例外
/// この関数は、エラーを返します。
/// ## 例
/// ```rust
/// use posts::api::get_post;
/// async {
///     let post = get_post(10000).await;
///     assert!(post.is_err());
/// };
/// ```
pub async fn get_post(id: u32) -> Result<Post, Box<dyn std::error::Error>> {
    let url = format!("{}/posts/{}", BASE_URL, id);
    let response = reqwest::get(&url).await?.json::<Post>().await?;
    Ok(response)
}

/// Post項目のリストを取得します。
/// # 戻り値
/// * `Result<Vec<Post>, Box<dyn std::error::Error>>` - Post項目のリストを返します。
/// * `Box<dyn std::error::Error>` - エラーを返します。
/// # 例外
/// * `std::error::Error` - エラーを返します。
/// # 詳細
/// この関数は、Post項目のリストを取得します。
/// ## 例
/// ```rust
/// use posts::api::get_posts;
/// async {
///     let posts = get_posts().await.unwrap();
///     assert!(!posts.is_empty());
///     assert_eq!(posts.len(), 100);
/// };
/// ```
/// # 例外
/// この関数は、エラーを返します。
/// ## 例
/// ```rust
/// use posts::api::get_posts;
/// async {
///     let posts = get_posts().await;
///     assert!(posts.is_err());
/// };
/// ```
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
