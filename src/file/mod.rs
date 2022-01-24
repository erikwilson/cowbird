use crate::log;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "file"));
}

pub fn copy(source: &str, target: &str) {
    println!("todo: implement copy: {:?} {:?}", source, target);
    info!(LOGGER,"copy"; "source" => source, "target" => target);
}

pub fn delete(file: &str) {
    println!("todo: implement delete: {:?}", file);
    info!(LOGGER,"delete"; "file" => file);
}
