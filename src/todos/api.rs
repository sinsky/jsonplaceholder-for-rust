use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct Todo {
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

pub async fn get_todo(id: u32) -> Result<Todo, Box<dyn std::error::Error>> {
    let url = format!("{}/todos/{}", BASE_URL, id);
    let response = reqwest::get(&url).await?.json::<Todo>().await?;
    Ok(response)
}

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
