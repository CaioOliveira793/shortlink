# Shortlink

A scalable URL shortener

## Usage

Under the `service` dir, a `docker-compose.yaml` file describes all the services needed to run the project

The development services are composed by:

- In memory SurrealDB instance
- Backend service

## Documentation

The service has a [Open API Specification](https://github.com/CaioOliveira793/shortlink/blob/main/service/openapi/main.yaml) describing all the endpoints. A rendered version can be found [here](https://petstore.swagger.io/?url=https://raw.githubusercontent.com/CaioOliveira793/shortlink/main/service/openapi/main.yaml)

## URL Shortening strategies

A URL shortener can be implemented in numerous ways, each of than can favor some aspect of the system (scalability, simplicity, cost, ...).

For comparison purposes, this project aims to implement multiple strategies using the rust language and the available database services on the market.

> At the moment, only the simplest short url generation strategy was implemented

### Long URL Random Number

TODO: explain the long url hash plus random numbers strategy

### Consistent Hash

### Distributed Counter

Distributed increment counter aggregated in the long url hash

### Long URL Unique Counter

Counter associated with the long url, incremented on each short url creation

