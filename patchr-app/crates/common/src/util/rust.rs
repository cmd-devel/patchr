use std::ops::ControlFlow;

pub fn to_unit<T>(_x: T) -> () {}

pub fn result_to_control_flow<U, E, O>(
    r: Result<U, E>, err_f: impl Fn(E) -> O,
) -> ControlFlow<O, U> {
    r.map(|p| ControlFlow::Continue(p))
        .unwrap_or_else(|e| ControlFlow::Break(err_f(e)))
}
