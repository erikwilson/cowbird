use crate::log;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "network"));
}

pub fn net_cat(source: &String, destination: &String) {
    println!("todo: implement net-cat: {:?} {:?}", source, destination);
    info!(LOGGER,"net_cat"; "source" => source, "destination" => destination);
}
