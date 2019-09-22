pub mod util {
    use std::io;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use rsfs::*;
    use rsfs::mem::FS;

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Create(PathBuf),
        List(Option<PathBuf>),
        Move(PathBuf, PathBuf),
        Delete(PathBuf)
    }

    impl FromStr for Command {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {

            let argv: Vec<&str> = s.split_whitespace().collect::<Vec<_>>();
            match &argv[..] {
                &["MOVE", from_dir, to_dir] => Ok(Command::Move(from_dir.into(), to_dir.into())),
                &["CREATE", target_dir] => Ok(Command::Create(target_dir.into())),
                &["LIST"] => Ok(Command::List(None)),
                &["LIST", target_dir] => Ok(Command::List(Some(target_dir.into()))),
                &["DELETE", target_dir] => Ok(Command::Delete(target_dir.into())),
                &_ => Err(())
            }
        }
    }

    pub struct VFS {
        fs: FS
    }

    fn print_path( dir: &PathBuf, root_dir: &PathBuf ) {
        let basename = dir.file_name().unwrap();
        let root_depth = root_dir.components().count();
        let dir_depth = dir.components().count();
        let indent = (dir_depth - root_depth - 1) * 2;
        println!("{:indent$}{}", "", basename.to_str().unwrap(), indent=indent);
    }

    impl VFS {

        pub fn new() -> VFS {
            let fs = FS::new();
            VFS { fs }
        }

        pub fn apply_command( &self, cmd: &Command ) -> io::Result<()> {
            match cmd {
                Command::Create(dir) => {
                    self.fs.create_dir_all(dir)
                },
                Command::List(Some(dir)) => {
                    self.print_hierarchy(&dir, &dir)
                }
                Command::List(None) => {
                    self.print_hierarchy(&"./".into(), &"./".into())
                },
                Command::Move(from, to) => {
                    let basename = from.file_name().unwrap();
                    let target = Path::new(to.to_str().unwrap()).join(basename);
                    self.fs.rename(from, target)
                },
                Command::Delete(dir) => {
                    self.fs.remove_dir_all(dir)
                }
            }
        }

        pub fn print_hierarchy( &self, dir: &PathBuf, root_dir: &PathBuf ) -> io::Result<()> {
            for entry in self.fs.read_dir(dir)? {
                let entry = entry?;

                if entry.file_type()?.is_dir() {
                    let path = entry.path();
                    print_path(&path, &root_dir);
                    self.print_hierarchy(&path, &root_dir)?;
                }
            }
            Ok(())
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::Command;
        use std::str::FromStr;

        #[test]
        pub fn it_parses() {
            let create_dir = Command::from_str("CREATE a/b/c").unwrap();
            assert_eq!(create_dir, Command::Create("a/b/c".into()));

            let move_dir = Command::from_str("MOVE a b").unwrap();
            assert_eq!(move_dir, Command::Move("a".into(), "b".into()));

            let list_dir_root = Command::from_str("LIST").unwrap();
            assert_eq!(list_dir_root, Command::List(None));

            let list_dir = Command::from_str("LIST /a").unwrap();
            assert_eq!(list_dir, Command::List(Some("/a".into())));

            let delete_dir = Command::from_str("DELETE /a").unwrap();
            assert_eq!(delete_dir, Command::Delete("/a".into()));
        }
    }
}
