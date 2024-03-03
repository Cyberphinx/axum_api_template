### Steps to initialize the web server
1. Setup a postgres database and fill in the DATABASE_URL in `.env` file
2. Designate a `PORT` in `.env` file
3. Generate a `JWT_SECRET` in `.env` using your favourite password generator
4. Comment out all sqlx queries in the code to avoid sqlx throwing errors
5. Run `cargo watch -x run` to run the server and apply pending migrations

### Steps to test the web server
1. GET request `curl http://localhost:PORT/api/v1/healthcheck` to do health check
2. Uncomment all sqlx queries after all pending migrations has been applied
3. API should be working to take CRUD requests using Postman to test following endpoints:
    4.1 POST request to `http://localhost:PORT/api/v1/signup` with JSON payload email and password to create a new user
    4.2 POST request to `http://localhost:PORT/api/v1/users/login` with previous credentials to login
    4.3 POST request to `http://localhost:PORT/api/v1/users/logout` with `Authorization` header and `bearer token` to logout
