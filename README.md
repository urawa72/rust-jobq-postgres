# rust-jobq-postgres

ref: https://github.com/skerkour/kerkour.com/tree/main/2021/rust_postgresql_job_queue

```bash
$ docker run --name rust_job_queue -d -e POSTGRES_USER=rust -e POSTGRES_PASSWORD=job_queue -p 5432:5432 postgres:latest
$ DATABASE_URL=postgres://rust:job_queue@localhost:5432/rust cargo run
$ docker rm -f rust_job_queue
```
