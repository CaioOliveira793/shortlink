# Shortlink

A scalable URL shortener

## Usage

Under the `service` dir, a `docker-compose.yaml` file describes all the services needed to run the project

The development services are composed by:

- In memory SurrealDB instance
- Backend service

## URL Shortening strategies

A URL shortener can be implemented in numerous ways, each of than can favor some aspect of the system (scalability, simplicity, cost, ...).

For comparison purposes, this project aims to implement multiple strategies using the rust language and the available database services on the market.

> At the moment, only the simplest short url generation strategy was implemented

### Long URL hash with random salt

TODO: explain the long url hash plus random salt strategy

