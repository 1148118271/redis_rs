pub struct Result<T> {
    pub state: State,
    pub value: T
}

pub enum State {
    OK,
    ERROR,
    NULL
}


impl Result<String> {
    pub fn new(state: State) -> Result<String> {
        Result {
            state,
            value: String::new()
        }
    }

    pub fn from(&mut self, v: String) {
        self.value = v;
    }
}