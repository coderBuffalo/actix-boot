# usage

### diesel

Diesel's [Getting Started](http://diesel.rs/guides/getting-started/) guide using MYSQL for Actix web

### setup mysql

```bash
cd actix-web
cargo install diesel_cli --no-default-features --features mysql
echo "DATABASE_URL=mysql://root@localhost:3306/actix_web" > .env
diesel migration run
```

### server

```bash
cd actix-web
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

[http://127.0.0.1:8080/NAME](http://127.0.0.1:8080/NAME)
