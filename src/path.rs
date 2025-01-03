pub mod metrics {
    use csv::Writer;
    use serde::Serialize;
    use serde_json;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap();

        path.push("target");
        path.push("metrics");

        path
    }

    pub fn mk_path(prefix: &[&str], name: &str) -> PathBuf {
        assert!(prefix.len() > 0);

        let mut path = path();
        for &name in prefix {
            path.push(name);
        }

        if !Path::exists(&path) {
            fs::create_dir_all(&path).unwrap();
        }

        path.push(name);
        path
    }

    pub fn store_csv(path: &Path, metrics: &[impl Serialize]) {
        let result_path = path;

        let mut wrt = Writer::from_path(result_path).expect("cannot create writer");
        for m in metrics.iter() {
            wrt.serialize(m).unwrap()
        }
        wrt.flush().unwrap();
    }

    pub fn store_json(path: &Path, metrics: &impl Serialize) {
        let bytes = serde_json::to_vec_pretty(metrics).expect("cannot map to json");
        fs::write(path, bytes).expect("cannot write");
    }
}
