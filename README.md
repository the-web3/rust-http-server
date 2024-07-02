<!--
parent:
  order: false
-->

<div align="center">
  <h1> Rust http server </h1>
</div>

<div align="center">
  <a href="https://github.com/the-web3/rust-http-server/releases/latest">
    <img alt="Version" src="https://img.shields.io/github/tag/the-web3/rust-http-server.svg" />
  </a>
  <a href="https://github.com/the-web3/rust-http-server/blob/main/LICENSE">
    <img alt="License: Apache-2.0" src="https://img.shields.io/github/license/the-web3/rust-http-server.svg" />
  </a>
  <a href="https://pkg.go.dev/github.com/the-web3/rust-http-server">
    <img alt="GoDoc" src="https://godoc.org/github.com/the-web3/rust-http-server?status.svg" />
  </a>
  <a href="https://goreportcard.com/report/github.com/the-web3/rust-http-server">
    <img alt="Go report card" src="https://goreportcard.com/badge/github.com/the-web3/rust-http-server"/>
  </a>
</div>

rust-http-server is a practical code of Rust http service

## Env
```bash
source .env
```

## Migrate database

```bash
diesel migration run
```

## Build 

```bash
cargo build
```

## Run

```bash
cargo run
```

## api example

### register
- request
```
curl --location 'http://127.0.0.1:8080/register' \
--header 'Content-Type: application/json' \
--data '{
    "id": "111101",
    "username": "seek011",
    "password": "123456"
}'
```

- response
```
{
    "id": "111101",
    "username": "seek011",
    "password": "$2b$12$VQg3lr.0k8bNiiS9x51MHePJ1fG.SlFPRSYAqzKrWfwEQuq7aaSte"
}
```

### login
- request
```
curl --location 'http://127.0.0.1:8080/login' \
--header 'Content-Type: application/json' \
--data '{
    "username": "seek010",
    "password": "123456"
}'
```
- response

```
"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMTEzIiwiZXhwIjoxMDAwMDAwMDAwMH0.W8e37C6fXhRdV9KEQSKuSDGn8iI3LngE_3PT11F9McU"
```

### Write blog
- request
```
curl --location 'http://127.0.0.1:8080/blog' \
--header 'Content-Type: application/json' \
--data '{

    "id": "111",
    "author_id": "111101",
    "title": "博客内容",
    "content": "博客内容博客内容博客内容博客内容博客内容博客内容博客内容"
}'
```
- response
```
{
    "id": "111",
    "title": "博客内容",
    "content": "博客内容博客内容博客内容博客内容博客内容博客内容博客内容",
    "author_id": "111101"
}
```
### Get blogs
- request
```
curl --location 'http://127.0.0.1:8080/blog/111' \
--data ''
```
- response
```
{
    "id": "111",
    "title": "博客内容",
    "content": "博客内容博客内容博客内容博客内容博客内容博客内容博客内容",
    "author_id": "111101"
}
```
