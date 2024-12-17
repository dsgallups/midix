macro_rules! io_error {
    ($kind:expr, $msg:expr) => {
        std::io::Error::new($kind, $msg)
    };
    ($msg:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $msg)
    };
}
