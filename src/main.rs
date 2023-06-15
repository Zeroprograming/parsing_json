use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("\n =========================== \n Starting... ");
    println!(" \n GET API Call");

    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:#?}", todos);

    println!("User ID: {}", todos[0].user_id);

    post().await?;

    post_serde_json().await?;

    Ok(())
}

async fn post() -> Result<(), reqwest::Error> {
    println!("\n =========================== \n Starting... ");
    println!(" \n\n POST API Call");

    let new_todo: Todo = Todo {
        user_id: 1,
        id: None,
        title: "Hello".to_string(),
        completed: false,
    };

    let res: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:#?}", res);
    Ok(())
}

async fn post_serde_json() -> Result<(), reqwest::Error> {
    println!("\n =========================== \n Starting... ");
    println!(" \n\n POST API Call with serde_json");

    let _res: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Hello".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    let lat: String = String::from("title");
    println!("{}", _res.get(&lat).to_owned().unwrap());

    println!("\n \n {:#?}", _res);

    Ok(())
}
