use crate::emulator::State8080;

pub fn setup_state() -> State8080 {
    let contents = vec![0; 64_000];
    let mut state =  State8080::new(contents);
    state.sp = 100;
    return state;
}