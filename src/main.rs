mod todos;
use crate::todos::api::{get_todo, get_todos};

mod posts;
use crate::posts::api::{get_post, get_posts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todos = get_todos().await.unwrap();
    println!("Todos length: {:#?}", todos.len());

    let todo = get_todo(1).await.unwrap();
    println!("id: 1 todo data");
    println!("---");
    println!("{:#?}", todo);
    println!("---");
    println!("todo id: {:#?}", todo.id);
    println!("user id: {:#?}", todo.user_id);
    println!("title: {:#?}", todo.title);
    println!("completed: {:#?}", todo.completed);

    println!("\n------------\n");

    let posts = get_posts().await.unwrap();
    println!("Posts length: {:#?}", posts.len());

    let post = get_post(10).await.unwrap();
    println!("id: 1 post data");
    println!("---");
    println!("{:#?}", post);
    println!("---");
    println!("post id: {:#?}", post.id);
    println!("user id: {:#?}", post.user_id);
    println!("title: {:#?}", post.title);
    println!("body: {:#?}", post.body);

    Ok(())
}
