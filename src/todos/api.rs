/// Module for managing todos.
/// Module for managing todos.
use serde::Deserialize;

/// jsonplaceholderのtodo構造
#[derive(Debug, Deserialize)]

pub struct Todo {
    /// * `u32` - Todo項目のID
    #[serde(rename = "userId")]
    pub user_id: u32,
    /// * `u32` - Todo項目のID
    pub id: u32,
    /// * `String` - Todo項目のタイトル
    pub title: String,
    /// * `bool` - Todo項目の完了状態
    pub completed: bool,
}

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

/// 指定されたIDを持つTodo項目を取得します。
/// # Arguments
/// * `id` - Todo項目のID
/// # Returns
/// * `Result<Todo, Box<dyn std::error::Error>>` - Todo項目を返します。
/// * `Box<dyn std::error::Error>` - エラーを返します.
/// # Details
/// この関数は、指定されたIDを持つTodo項目を取得します。
/// ## Example
/// ```rust
/// use todos::api::get_todo;
/// async {
///     let todo = get_todo(1).await.unwrap();
///     assert_eq!(todo.id, 1);
/// };
/// ```
/// # Errors
/// この関数は、エラーを返します。
/// ## Example
/// ```rust
/// use todos::api::get_todo;
/// async {
///     let todo = get_todo(10000).await;
///     assert!(todo.is_err());
/// };
/// ```
/// use todos::api::get_todo;
/// async {
///     let todo = get_todo(10000).await;
///     assert!(todo.is_err());
/// };
/// ```
/// 指定されたIDを持つTodo項目を取得します。
/// # Arguments
/// * `id` - Todo項目のID
/// # Returns
/// * `Result<Todo, Box<dyn std::error::Error>>` - Todo項目を返します。
/// * `Box<dyn std::error::Error>` - エラーを返します.
/// # Details
pub async fn get_todo(id: u32) -> Result<Todo, Box<dyn std::error::Error>> {
    let base_url = BASE_URL;
    let url = format!("{}/todos/{}", base_url, id);
    let response = reqwest::get(&url).await?.json::<Todo>().await?;
    Ok(response)
}

/// Todo項目のリストを取得します。
/// # 戻り値
/// * `Result<Vec<Todo>, Box<dyn std::error::Error>>` - Todo項目のリストを返します。
/// * `Box<dyn std::error::Error>` - エラーを返します。
/// # 例外
/// * `std::error::Error` - エラーを返します。
/// # 詳細
/// この関数は、Todo項目のリストを取得します。
/// ## 例
/// ```rust
/// use todos::api::get_todos;
/// async {
///     let todos = get_todos().await.unwrap();
///     assert!(!todos.is_empty());
///     assert_eq!(todos.len(), 200);
/// };
/// ```
/// # 例外
/// この関数は、エラーを返します。
/// ## 例
/// ```rust
/// use todos::api::get_todos;
/// async {
///     let todos = get_todos().await;
///     assert!(todos.is_err());
/// };
/// ```
pub async fn get_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let url = format!("{}/todos", BASE_URL);
    let response = reqwest::get(url).await?.json::<Vec<Todo>>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_todo() {
        // Arrange
        let id = 1;
        // Act
        let result = get_todo(id).await;
        // Assert
        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.id, id);
    }
    #[tokio::test]
    async fn test_get_todo_empty() {
        // Arrange
        let id = 10000;
        // Act
        let result = get_todo(id).await;
        // Assert
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_get_todos() {
        // Act
        let result = get_todos().await;
        // Assert
        assert!(result.is_ok());
        let todos = result.unwrap();
        assert!(!todos.is_empty());
        assert_eq!(todos.len(), 200);
    }
}
use mockito::{mock, server_url};
use super::api_doc;
    #[tokio::test]
    async fn test_get_todo_network_error() {
        let _m = mock("GET", "/todos/1")
            .with_status(500)
            .create();

        let result = get_todo(1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_todo_unexpected_response() {
        let _m = mock("GET", "/todos/1")
            .with_status(200)
            .with_body("unexpected response format")
            .create();

        let result = get_todo(1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_todos_network_error() {
        let _m = mock("GET", "/todos")
            .with_status(500)
            .create();

        let result = get_todos().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_todos_unexpected_response() {
        let _m = mock("GET", "/todos")
            .with_status(200)
            .with_body("unexpected response format")
            .create();

        let result = get_todos().await;
        assert!(result.is_err());
    }
