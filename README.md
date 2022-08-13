# zero2prod

- [Foreword \| A learning journal](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/)
- https://github.com/LukeMathWalker/zero-to-production

## Development

```sh
# requirements
$ cargo install sqlx-cli
$ cargo install cargo-watch

# run development server
$ docker compose up -d
$ make db_setup
$ cargo watch -x run

# run test
$ make test_db_setup
$ cargo test
```
