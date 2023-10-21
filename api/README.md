<div align="center">
    <img src="../common/assets/logo.png" alt="Marites" width="100">
    <hr />
        <h2 align="center" style="border-bottom: none">Marites API</h2>
        <img alt="GitHub" src="https://img.shields.io/github/license/txpipe/oura" />
        <img alt="Crates.io" src="https://img.shields.io/crates/v/oura" />
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/txpipe/oura/validate.yml" />
    <hr/>
</div>

## Quick Start (watch)
```sh
# Terminal 1 - To run server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run examples
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Starting the DB
```sh
# Start postgesql server docker image:
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

# (optional) Run a psql terminal on pg. Open a new terminal tab:
docker exec -it -u postgres pg psql
```

## Unit Test (watch)
```sh
cargo watch -q -c -x "test -- --nocapture"

# Filter tests
cargo watch -q -c -x "test::model::task::tests::test_create_ok()"
```

## Status
It's currently just an idea and there's not much development progress. You're welcome to contribute. You can send me an email at rjlacanlaled@gmail.com if you're interested.