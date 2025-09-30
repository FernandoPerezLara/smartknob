#[macro_export]
macro_rules! include_generated {
    (bytes: $filename:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $filename))
    };
    (str: $filename:expr) => {
        include_str!(concat!(env!("OUT_DIR"), "/", $filename))
    };
}
