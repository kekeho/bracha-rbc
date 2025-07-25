# Rust implementation of Bracha's RBC

## How to execute

Boot Example:

```sh
(TERMINAL 1)$ cargo run honest 127.0.0.1:8000 127.0.0.1:8001 127.0.0.1:8002 127.0.0.1:8003
(TERMINAL 2)$ cargo run honest 127.0.0.1:8001 127.0.0.1:8000 127.0.0.1:8002 127.0.0.1:8003
(TERMINAL 3)$ cargo run honest 127.0.0.1:8002 127.0.0.1:8000 127.0.0.1:8001 127.0.0.1:8003
(TERMINAL 4)$ cargo run byzantine 127.0.0.1:8003 127.0.0.1:8000 127.0.0.1:8001 127.0.0.1:8002
```

and type some message in each terminal...

## Demo

https://github.com/user-attachments/assets/9d6060f6-5d94-49ed-8d15-eea8a09ea79c




## What is Bracha's RBC?

See [famous blog article](https://decentralizedthoughts.github.io/2020-09-19-living-with-asynchrony-brachas-reliable-broadcast/) and [this video](https://www.youtube.com/watch?v=T0-o3s7bibw).
