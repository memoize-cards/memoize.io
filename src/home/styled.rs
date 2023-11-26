use super::footer;
use super::header;
use super::main;

pub fn init() {
    header::styled::init();
    main::styled::init();
    footer::styled::init();
}
