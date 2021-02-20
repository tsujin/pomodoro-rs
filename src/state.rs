enum State {
    Menu,
    Running,
    Paused,
    Break,
}

struct PomodoroState {
    prev_state: State,
    curr_state: State,
}

impl PomodoroState {
    fn init() -> PomodoroState {
        PomodoroState {
            prev_state: None,
            curr_state: Menu,
        }
    }

    fn set_state(&mut self, state: State) {
        self.prev_state = curr_state;
        self.curr_state = state;
    }
}
