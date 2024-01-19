use crate::kprint;
use crate::time;

// TODO introduce log levels
// TODO then also add colors for levels (Error --> red)
pub fn log(text: &str) {
    kprint::kprint_char('[');
    time::kprint_time();
    kprint::kprint("] ");
    kprint::kprint_line(text);
}
