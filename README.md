# Mud Engine
<!-- TOC -->

- [Mud Engine](#mud-engine)
    - [Description](#description)
    - [Building](#building)
    - [Running](#running)
        - [MUD Only](#mud-only)
        - [Full-Stack](#full-stack)

<!-- /TOC -->
## Description
The purpose of the mud engine is to provide a comparable tool to LambdaMOO or other interactive engines for building a mud.

## Building
The engine uses docker for builds and can be built by running the following command.

```
docker build -t mud .
```

This will return a minified image containing the MUD's binary.

## Running
### MUD Only
After running through the [build](#build) stage, you can run the container via docker by exposing the port 3012.

```
docker run -p 3012:3012 mud
```

It will be accessible using any websocket client.

### Full-Stack
The project includes a minimal webclient and nginx server/proxy which can be stood up in a docker environment via docker-compose. Running the following in the project root is all that is required to build and start the entire project.

```
docker-compose up
```

After doing so, everything will be accessible on port 80.