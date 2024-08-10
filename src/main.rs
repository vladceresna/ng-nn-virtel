mod files;
mod apps;
mod system;
mod logger;
mod step;
mod launcher;
mod time;

fn main() {
    match launcher::launch() {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}
