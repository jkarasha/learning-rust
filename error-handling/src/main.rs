#[derive(Debug)]
enum Platform {
    Windows,
    Linux,
    MacOS
}

impl Platform {
    fn parse(platform_arg: &str) -> Platform {
        if platform_arg == "windows" {
            Platform::Windows
        } else if platform_arg == "linux" {
            Platform::Linux
        } else if platform_arg == "mac" {
            Platform::MacOS
        } else {
            panic!("unknown platform: {}. Valid values: [windows, linux, mac].", platform_arg);
        }
    }
}

fn main() {
    //defaults to windows..
    let platform_arg = "macos";
    let platform = Platform::parse(platform_arg);
    println!("Producing output for {:?}.", platform);
}