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
/// # 引数
/// * `id` - Todo項目のID
/// # 戻り値
/// * `Result<Todo, Box<dyn std::error::Error>>` - Todo項目を返します。
/// * `Box<dyn std::error::Error>` - エラーを返します。
/// # 例外
/// * `std::error::Error` - エラーを返します。
/// # 詳細
/// この関数は、指定されたIDを持つTodo項目を取得します。
/// ## 例
/// ```rust
/// use todos::api::get_todo;
/// async {
///     let todo = get_todo(1).await.unwrap();
///     assert_eq!(todo.id, 1);
/// };
/// ```
/// # 例外
/// この関数は、エラーを返します。
/// ## 例
/// ```rust
/// use todos::api::get_todo;
/// async {
///     let todo = get_todo(10000).await;
///     assert!(todo.is_err());
/// };
/// ```
pub async fn get_todo(id: u32) -> Result<Todo, Box<dyn std::error::Error>> {
    let url = format!("{}/todos/{}", BASE_URL, id);
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
