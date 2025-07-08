use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use std::io::BufReader;
use std::sync::Once;
use std::{fs::File, io::Read};

use cafebabe::{ClassFile, read_class_data};

static LOGGING_STATE: Once = Once::new();

pub fn setup_logging() {
    LOGGING_STATE.call_once(|| {
        let logger = TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
        CombinedLogger::init(vec![logger]).unwrap();
    });
}

pub fn read_class_from_path(path: &str) -> ClassFile {
    let file = File::open(path).expect("Can't open class file");
    let mut reader = BufReader::new(file);
    let mut data: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut data)
        .expect("Can't read class file into memory");

    read_class_data(&data).expect("Can't parse class file")
}

pub fn validate_class_name(class_file: &ClassFile, expected_class_name: &str) {
    let class_ref_idx = class_file.class.this_idx;
    let class_name_idx = class_file
        .constant_pool
        .class_ref_entry(class_ref_idx)
        .unwrap();
    let class_name = class_file
        .constant_pool
        .string_entry(class_name_idx)
        .unwrap();
    assert_eq!(class_name, expected_class_name, "Expect class to be valid");
}
