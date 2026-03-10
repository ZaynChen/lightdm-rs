use lightdm::prelude::{GreeterExtManual, LanguageExt};

fn main() {
    let languages = lightdm::functions::languages();
    languages.iter().for_each(|l| println!("{:?}", l.code()));

    let greeter = lightdm::Greeter::new();
    if let Err(e) = greeter.connect_to_daemon_sync() {
        println!("ERROR: {e}");
        return;
    }

    if let Err(e) = greeter.authenticate(None) {
        println!("ERROR: {e}");
    }
}
