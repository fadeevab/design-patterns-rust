# Proxy

_**Proxy** is a structural design pattern that provides an object that acts as a
substitute for a real service object used by a client. A proxy receives client
requests, does some work (**access control, caching**, etc.) and then passes the
request to a service object._

## Conceptual Example: Nginx Proxy

A web server such as Nginx can act as a proxy for your application server:

- It provides controlled access to your application server.
- It can do rate limiting.
- It can do request caching.

## How to Run

```bash
cargo run --bin proxy
```

## Execution Result

```
Url: /app/status
HttpCode: 200
Body: Ok

Url: /app/status
HttpCode: 200
Body: Ok

Url: /app/status
HttpCode: 403
Body: Not Allowed

Url: /create/user
HttpCode: 201
Body: User Created

Url: /create/user
HttpCode: 404
Body: Not Ok
```

## Reference

This example reproduces a [Proxy Example in Go](https://refactoring.guru/design-patterns/proxy/go/example).
