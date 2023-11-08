use crate::print;
use crate::time;

// TODO introduce log levels
// TODO then also add colors for levels (Error --> red)
pub fn log(text: &str) {
    print::print_char('[');
    time::print_time();
    print::print("] ");
    print::print_line(text);
}
