# leptos-test
Test leptos framework to assist w/ isaac's scouting app

Blue alliance api keys:
- https://www.thebluealliance.com/account

Leptos Framework book
- https://github.com/leptos-rs/book

# Local Development

To serve the docker build locally run: 
```
$ docker-compose up
$ docker-compose up --build --detach # to rebuild
```

To stop container without removing it:
```
$ docker-compose down
```

To delete all existing containers, networks, volumes, and images
```
$ docker-compose down --volumes --rmi all
```

# Deployment

Build image locally:
```
$ docker build -t test-this .
```

List images in your local docker application:
```
$ docker image ls
```

How to run the docker container locally:
```
$ docker run -p 3000:3000 leptos-test
$ docker run -p 127.0.0.1:3000:3000 leptos-test
```
