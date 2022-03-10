# Integration tests written in Go

## Prerequisities

The following instructions apply to Linux.

### Install Go

Download Go installation files for Linux: https://golang.google.cn/dl/go1.17.8.linux-amd64.tar.gz

Follow the installation instructions for Linux: https://golang.google.cn/doc/install

Check the installed Go version:

```
$ go version
go version go1.17.8 linux/amd64
```

### Install IDE

Install GoLand from JetBrains or any editor of your choice.

## Installing dependencies

Tests for `nordnotes` back-end application depend on testing framework and some other libraries.
To install these dependencies run `deps.sh` script:

```
$ ./deps.sh
Removing existing dependencies...
Downloading newest dependencies...
Getting: uuid
Getting: jose
Getting: oxyde
Downloading dependencies completed.
```

## Building tests

To build testing application run `build.sh` script:

```
$ ./build.sh
```

## Running tests

To run all tests start the tested back-end application and then execute command:

```
$ ./bin/tests ALL
>>> nordnotes                                                                                                           v1   
 >> ts_service                                                                                                          v1   
  > td_service_info                                                                                                     v1   
    - tc_service_info []                                                                                                v1   OK
Documentation server started and listening on port: 16100
```

Details of the testing process (test suites, documentation tests and test cased) are printed to standard output.

## Documenting tests

When all tests pass, then automatically generated documentation of the tested application is available under the link:
```
http:://localhost:16100
```