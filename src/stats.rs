use std::io::Result;
use std::

pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;
    if !silent {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
    // if !silent {
    //     eprintln!("\r{}", total_bytes);
    // }
}
