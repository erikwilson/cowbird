use crate::log;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "process"));
}

pub fn exec(executable: &str, args: &[String]) {
    println!("todo: implement exec: {:?} {:?}", executable, args);
    info!(LOGGER,"exec"; "executable" => executable);
}
