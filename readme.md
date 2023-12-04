# SpeedCalc
SpeedCalc is a demo web application consisting of a front-end made with vue.js using Typescript, Tailwind CSS and a back-end server written in Rust using actix-web. The server generates simple math tasks that get displayed in the front-end and should be solved by the user. The answer is evaluated in the back-end via a websocket connection to minimize delays.

# Setup
```
cd client
npm run build
cd ..
cargo build
```

# Running the server
```
cargo run   # use -r to run in release mode
```

# Desktop
There is also a desktop application provided by tauri. Make sure that the tauri CLI is installed and the server is running (see above).
## Running the desktop application
```
cd client
cargo tauri dev
```
## Building a desktop app
```
cd client
cargo tauri build
```
