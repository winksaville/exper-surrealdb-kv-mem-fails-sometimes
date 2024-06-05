use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() {
    println!("entering");
    let db = match Surreal::new::<Mem>(()).await {
        Ok(db) => db,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    match db.version().await {
        Ok(v) => println!("version: {}", v),
        Err(e) => println!("version failed: {}", e),
    }

    match db.health().await {
        Ok(_) => println!("Health check passed"),
        Err(e) => println!("Health check failed: {}", e),
    }

    println!("exiting");
}
