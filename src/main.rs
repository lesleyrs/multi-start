use std::{
    collections::HashMap,
    env, fs,
    path::Path,
    process::{self, Command},
};

use serde_json::{from_str, Value};
use time::{OffsetDateTime, UtcOffset};

const NAME: &str = env!("CARGO_BIN_NAME");
const EPOCH: i64 = 11644473600000000;
const MICROSECONDS: i64 = 1000000;

fn main() {
    #[allow(deprecated)] // home_dir works well enough
    let chrome_bookmarks = fs::read_to_string(format!(
        r"{}\AppData\Local\Google\Chrome\User Data\Default\Bookmarks",
        env::home_dir().unwrap().display()
    ))
    .unwrap();

    let json: HashMap<String, Value> = from_str(&chrome_bookmarks).unwrap();

    match env::args().nth(1) {
        Some(arg) if arg == "ls" || arg == "list" => {
            // filter folders (couldn't make them work properly)
            for (i, j) in json["roots"]["bookmark_bar"]["children"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|x| x["url"].as_str().is_some())
                .enumerate()
            {
                // https://stackoverflow.com/questions/539900/google-bookmark-export-date-format#comment87155880_18685018
                // https://stackoverflow.com/a/51343829

                if j["url"].as_str().is_some() {
                    println!(
                        "#{} - \x1b[30m\x1b[43m{}\x1b[0m - \x1b[92madded\x1b[0m: {} \x1b[92mlast used\x1b[0m: {}\n\x1b[91m{}\x1b[0m\n",
                        i,
                        j["name"].as_str().unwrap(),
                        OffsetDateTime::from_unix_timestamp(
                            (j["date_added"].as_str().unwrap().parse::<i64>().unwrap() - EPOCH)
                                / MICROSECONDS
                        )
                        .unwrap()
                        .to_offset(
                            UtcOffset::local_offset_at(
                                OffsetDateTime::from_unix_timestamp(
                                    (j["date_added"].as_str().unwrap().parse::<i64>().unwrap()
                                        - EPOCH)
                                        / MICROSECONDS
                                )
                                .unwrap()
                            )
                            .unwrap()
                        ),
                        OffsetDateTime::from_unix_timestamp(
                            (j["date_last_used"]
                                .as_str()
                                .unwrap()
                                .parse::<i64>()
                                .unwrap()
                                - EPOCH)
                                / MICROSECONDS
                        )
                        .unwrap()
                        .to_offset(
                            UtcOffset::local_offset_at(
                                OffsetDateTime::from_unix_timestamp(
                                    (j["date_last_used"]
                                        .as_str()
                                        .unwrap()
                                        .parse::<i64>()
                                        .unwrap()
                                        - EPOCH)
                                        / MICROSECONDS
                                )
                                .unwrap()
                            )
                            .unwrap()
                        ),
                        j["url"].as_str().unwrap()
                    );
                }
            }
        }
        // this matches all but only uses ints, have to do other matching prior to this or try clap?
        Some(_arg) => {
            let args: Vec<String> = env::args().skip(1).collect();
            let mut vec: Vec<&str> = Vec::new();
            for i in json["roots"]["bookmark_bar"]["children"]
                .as_array()
                .unwrap()
                .iter()
            {
                if i["url"].as_str().is_some() {
                    vec.push(i["url"].as_str().unwrap());
                }
            }
            for arg in args {
                if let Ok(i) = arg.parse::<i32>() {
                    match i < vec.len().try_into().unwrap() {
                        true => {
                            if cfg!(target_os = "windows") {
                                let paths = [
                                    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                                    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                                ];

                                let chrome_path =
                                    paths.iter().find(|path| Path::new(path).exists());

                                match chrome_path {
                                    Some(path) => {
                                        println!(
                                            "Opening \x1b[93m{}\x1b[0m with \x1b[92m{}\x1b[0m",
                                            vec[i as usize], path
                                        )
                                    }
                                    None => {
                                        eprintln!("\x1b[91mGoogle Chrome not found.\x1b[0m");
                                        std::process::exit(1);
                                    }
                                }
                                Command::new("cmd")
                                    .args([
                                        "/C",
                                        chrome_path.unwrap(),
                                        &vec[i as usize].replace('&', "^&"),
                                    ])
                                    .spawn()
                                    .expect("failed to execute process")
                            } else {
                                Command::new("sh")
                                    .arg("-c")
                                    .arg("echo hello")
                                    .spawn()
                                    .expect("failed to execute process")
                            };
                            // if !output.stdout.is_empty() {
                            //     println!(
                            //         "output: {}",
                            //         std::str::from_utf8(&output.stdout).unwrap()
                            //     );
                            // }
                        }
                        false => match vec.len() {
                            0 => {
                                eprintln!("Error: no bookmarks found, folders are ignored.")
                            }
                            _ => eprintln!(
                                "Error: entered: [{}] max: [{}], folders are ignored.",
                                i,
                                vec.len() - 1
                            ),
                        },
                    }
                } else {
                    eprintln!(
                        "Error: Invalid arg(s) entered, try \"{} ls\" to show accepted numbers.",
                        NAME
                    );
                    eprintln!(
                "Usage: {} [space seperated numbers] e.g \"{} 0 1 2 3\" opens up bookmark 0 to 3.", NAME, NAME
            );
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!(
                "Error: No args entered, try \"{} ls\" to show accepted numbers.",
                NAME
            );
            eprintln!(
                "Usage: {} [space seperated numbers] e.g \"{} 0 1 2 3\" opens up bookmark 0 to 3.",
                NAME, NAME
            );
        }
    }
}
