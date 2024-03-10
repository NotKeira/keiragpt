use std::ops::ControlFlow;

pub fn execute() -> ControlFlow<(), String> {
    ControlFlow::Break(())
}
