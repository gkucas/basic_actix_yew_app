# Rust user application

## Project layout
* server - directory contains Actix based application managing users and serving static content 
* app - directory contains Yew based single page UI application

Project is configured to run as a single service where Actix web serves provides all content. To run application as separate
additional configuration would be required. For backend location, CORS policy and cookie policies.

When started application will be running on http://localhost:8000.

## Running the application prerequisites

* Cargo(tested with 1.77.2)
* Trunk(tested with 0.19.3) 

### Run in docker

```shell
trunk build app/index.html
docker-compose -f docker/docker-compose.yml build
docker-compose -f docker/docker-compose.yml up -d
```

### Run in development mode

* Start the database using docker-compose
```shell
docker-compose -f docker/docker-compose.yml up -d postgres
```
* From project root in your terminal run
```shell
cargo watch -w server -w src -x "run"<code>
```
* Open second terminal and go to `app` directory
```shell
trunk build
trunk watch
```