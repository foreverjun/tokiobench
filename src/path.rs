pub mod metrics {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap();

        path.push("target");
        path.push("metrics");

        path
    }

    pub fn mk_prefix(folder: &str) -> PathBuf {
        let mut path = path();
        path.push(folder);

        if !Path::exists(&path) {
            fs::create_dir_all(&path).unwrap();
        }

        path
    }

    pub fn store(prefix: &Path, name: &str, data: &[u8]) {
        let result_path = {
            let mut prefix = PathBuf::from(prefix);
            prefix.push(name);
            prefix
        };

        let mut f = File::create(result_path).unwrap();
        f.write_all(data).unwrap();
    }
}
