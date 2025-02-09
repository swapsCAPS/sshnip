# Clipboard server

Stores and serves data using api-key header auth.

## Run

```bash
ROCKET_API_KEY="hunter2" cargo run
```

## Usage

### POST

Store something.

```bash
curl \
  -X POST \
  -H 'x-api-key: hunter2' \
  http://localhost:8000/clip \
  -d 'henk'
```

### GET

Get what is stored and put in OSX clipboard. For Linux use something like `xclip` instead.

```bash
curl \
  -X GET \
  -H 'x-api-key: hunter2' \
  http://localhost:8000/clip \
  | pbcopy
```

## Test

```bash
ROCKET_API_KEY="hunter2" cargo test
```
