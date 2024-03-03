use api::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Missing env PORT")
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // check the existence of .env file
    let database_uri = match dotenvy::dotenv() {
        Ok(_) => std::env::var("DATABASE_URL").expect("Missing env DATABASE_URL in development"),
        Err(_) => {
            // fly io
            // if .env file doesnt exist, it means that it is running on fly.io
            std::env::var("DATABASE_URL")
                .expect("Missing env DATABASE_URL in production")
                .replace("flycast", "internal")
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET").expect("to have jwt secret");

    let environment = std::env::var("ENVIRONMENT").expect("Missing env ENVIRONMENT");

    let app = App::new(port, &database_uri, jwt_secret, &environment)
        .await
        .expect("Error creating server");

    app.run().await.expect("Error running server");
}
