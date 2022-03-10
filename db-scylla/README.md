# ScyllaDB

NoSQL database for `nordnotes` back-end application.

## Prerequisities

The following instructions apply to Linux.

### Install Docker

Install Docker following the instructions: https://docs.docker.com/engine/install/

Check installed Docker version:

```
$ docker -v
Docker version 20.10.12, build e91ed57
```

### Install Docker Compose

Install Docker Compose following the instructions: https://docs.docker.com/compose/install/

Check installed Docker Compose version:

```
$ docker-compose -v
docker-compose version 1.29.2, build 5becea4c
```

## Run ScyllaDB

To run ScyllaDB execute command:

```
$ docker-compose up -d
Creating scylla ... done
```

Wait until the database servcer is ready to process incoming requests, follow the logs:

```
$ docker-compose logs -f
...
scylla    | INFO  2022-03-10 07:23:12,572 [shard 0] init - Scylla version 4.5.4-0.20220220.53b0aaa4e initialization completed.
...
```

Check the ports:

```
$ docker ps
CONTAINER ID   IMAGE                    COMMAND                  CREATED         STATUS         PORTS                                                                                             NAMES
35a9cf3805c1   scylladb/scylla:latest   "/docker-entrypoint.…"   2 minutes ago   Up 2 minutes   22/tcp, 7000-7001/tcp, 9160/tcp, 9180/tcp, 10000/tcp, 0.0.0.0:9042->9042/tcp, :::9042->9042/tcp   scylla
```

ScyllaDB is ready.