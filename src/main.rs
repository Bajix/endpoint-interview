use std::io;
use std::io::ErrorKind;

use endpoint::util::{Command, VFS};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let input = include_str!("sample.txt");

    let vfs = VFS::new();

    for line in input.lines() {
        println!("{}", line);

        if let Ok(cmd) = Command::from_str(&line[..]) {
            let res = vfs.apply_command(&cmd);
            if let Err(e) = res {
                if e.kind() == ErrorKind::NotFound {
                    match cmd {
                        Command::Delete(dir) => {
                            let path = dir.to_str().unwrap();
                            let basename = dir.file_name().unwrap().to_str().unwrap();
                            println!("Cannot delete {} - {} does not exist", path, basename);
                        },
                        Command::Move(dir, _) => {
                            let path = dir.to_str().unwrap();
                            let basename = dir.file_name().unwrap().to_str().unwrap();
                            println!("Cannot move {} - {} does not exist", path, basename);
                        },
                        _ => panic!(e.to_string())
                    };
                } else {
                    panic!(e.to_string());
                }
            }
        } else {
            panic!("Invalid command: {}", line);
        }
    }

    Ok(())
}
