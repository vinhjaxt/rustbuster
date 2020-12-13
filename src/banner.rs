use chrono::Local;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn generate() -> String {
    format!("                   _    _                  _              
 _ __  _   _  ___ | |_ | |__   _   _  ___ | |_  ___  _ __ 
| '__|| | | |/ __|| __|| '_ \\ | | | |/ __|| __|/ _ \\| '__|
| |   | |_| |\\__ \\| |_ | |_) || |_| |\\__ \\| |_|  __/| |   
|_|    \\__,_||___/ \\__||_.__/  \\__,_||___/ \\__|\\___||_|   
")
}

pub fn copyright() -> String {
    format!(
        "~ rustbuster v{} ~ by phra & ps1dr3x ~
",
        VERSION
    )
}

/*
pub fn configuration(mode: &str, url: &str, threads: &str, wordlist: &str) -> String {
    format!(
        "[+] Mode\t: {}
[+] Url/Domain\t: {}
[+] Threads\t: {}
[+] Wordlist\t: {}",
        mode, url, threads, wordlist
    )
}
// */

pub fn starting_time() -> String {
    format!(
        "[?] Started at\t: {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn ending_time() -> String {
    format!(
        "\n[?] Ended at: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}
