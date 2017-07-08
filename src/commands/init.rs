#[cfg(test)]
extern crate tempdir;

use options::Options;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod templates {
    pub const DETECT: &'static str = "
#!/usr/bin/env bash
# bin/detect <build-dir>

echo \"Hello World\"
exit 0
";
    pub const COMPILE: &'static str = "
#!/usr/bin/env bash
# bin/compile <build-dir> <cache-dir>

echo \"-----> Nothing to do.\"
";
    pub const RELEASE: &'static str = "
#!/usr/bin/env bash

echo \"--- {}\"
";
}

pub struct Init {
    name: String,
}

impl Init {
    pub fn new(options: Options) -> Init {
        Init { name: options.arg_name }
    }

    pub fn execute(self) -> Result<(), io::Error> {
        let mut path_buf = env::current_dir()?;
        path_buf.push(self.name);
        fs::create_dir(path_buf.as_path())?;

        path_buf.push("bin");
        fs::create_dir(path_buf.as_path())?;

        path_buf.push("detect");
        let mut detect = fs::File::create(path_buf.as_path())?;
        detect.write_all(self::templates::DETECT.as_bytes())?;

        path_buf.pop();
        path_buf.push("compile");
        let mut compile = fs::File::create(path_buf.as_path())?;
        compile.write_all(self::templates::COMPILE.as_bytes())?;

        path_buf.pop();
        path_buf.push("release");
        let mut release = fs::File::create(path_buf.as_path())?;
        release.write_all(self::templates::RELEASE.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use self::tempdir::TempDir;
    use std::path;

    fn find_and_assert_dir<P: AsRef<path::Path>>(path: P, name: &'static str) -> fs::DirEntry {
        let mut read_dir = fs::read_dir(path).unwrap();

        let option = read_dir.find(|result| {
            let dir_entry = result.as_ref().unwrap();

            dir_entry.file_name().into_string().unwrap() == name
        });
        assert!(option.is_some());

        option.unwrap().unwrap()
    }

    #[test]
    fn execute() {
        let cmd = Init { name: String::from("buildpack") };
        let tmpdir = TempDir::new("test").unwrap();
        env::set_current_dir(tmpdir.path()).unwrap();

        println!("tmpdir: {}", tmpdir.path().display());

        cmd.execute().unwrap();

        let bp_dir = find_and_assert_dir(tmpdir.path(), "buildpack");
        let bin_dir = find_and_assert_dir(bp_dir.path(), "bin");

        // check files inside bin/ exist
        let expected_bin_files: Vec<String> = vec!["detect", "compile", "release"]
            .into_iter()
            .map(|str| {
                String::from(str)
            })
            .collect();
        let bin_dir_files: Vec<String> = fs::read_dir(bin_dir.path())
            .unwrap()
            .map(|result| {
                result.unwrap().file_name().into_string().unwrap()
            })
            .collect();
        assert!(expected_bin_files.into_iter().all(|file| {
            bin_dir_files.contains(&file)
        }));

        tmpdir.close().unwrap();
    }
}
