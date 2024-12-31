enum State {
    Initializing,
    Ready,
    Serving,
    Failed,
    ShuttingDown,
}

struct Server {
    state: State,
}