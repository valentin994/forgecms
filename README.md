ForgeCMS
---
⚡A lightning fast rust based CMS. ⚡

## Installation
The CMS is built with axum for managing the requests, and with sqlx & slqx-cli for managing the database.
Out of the box there will be a review table that is initilized with the server. There is a docker compose
set up to bootstrap everything, but if you already have a setup database you can run the server independently
wih: 

```sh
$ cd ./backend
$ cargo run --release
```

Or if you want to bootstrap from scratch use:
```sh
$ docker compose up
```
If you want the containers to run in the background run with `-d` which is detached mode:
```sc
$ docker compose up -d
```

To strip everything down run:
```sh 
$ docker compose down
```

If you do any changes to the code and want to apply those changes, run:
```sh 
$ docker compose build
$ docker compose up
```

## Database
This project uses postgres as its database of choice. 
When starting the application you will have these tables created:
- reviews

If you want to add more tables you can do that with the [sqlx-cli](https://crates.io/crates/sqlx-cli). 


## Contributions
If you want to contribute you are more than welcome. Please open a pull request and write tests for your PR.
After all tests and conditions pass I'll take a look into it and merge it.

