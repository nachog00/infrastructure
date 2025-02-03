use crate::utils::get_out_dir;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn get_binaries_dir() -> PathBuf {
    let out_dir = get_out_dir();
    out_dir.join("test_binaries")
}

// The following functions are only to try out the behaviour of files in OUT_DIR and accesing them from other crates
pub fn create_test_file_with_parents() {
    let file_path = get_test_file_path();

    // Create the parent directory if it does not exist
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directories");
    }

    // Create the file
    let mut file: File = File::create(file_path).expect("file to be created/read");
    file.write_all(b"Dummy content")
        .expect("file content to be written");
}

pub fn get_test_file_path() -> PathBuf {
    let file_path = get_binaries_dir().join("dummy_file");
    // Example file path
    file_path
}

#[derive(Debug, Clone)]
pub enum TestingBinaryVariant {
    Lightwalletd,
    Zainod,
    ZcashCli,
    Zcashd,
    Zebrad,
    ZingoCli,
}

#[derive(Debug)]
pub struct TestingBinary {
    variant: TestingBinaryVariant,
    // expected_bytes: [u8; 64],
    // version_string: String,
}

impl TestingBinary {
    pub fn new(variant: TestingBinaryVariant) -> Self {
        Self {
            variant,
            // expected_bytes,
            // version_string,
        }
    }

    pub fn get_name(&self) -> &str {
        match &self.variant {
            TestingBinaryVariant::Lightwalletd => "lightwalletd",
            TestingBinaryVariant::Zainod => "zainod",
            TestingBinaryVariant::ZcashCli => "zcash-cli",
            TestingBinaryVariant::Zcashd => "zcashd",
            TestingBinaryVariant::Zebrad => "zebrad",
            TestingBinaryVariant::ZingoCli => "zingo-cli",
        }
    }

    // Create the asset URL based on the binary variant
    pub fn get_asset_url(&self) -> String {
        format!("https://zingolabs.nexus:9073/{}", self.get_name())
    }

    /// Returns the get expected bytes of this [`TestingBinary`].
    // Expected bytes for quick checking of the binaries
    pub fn get_expected_bytes(&self) -> [u8; 64] {
        match &self.variant {
            TestingBinaryVariant::Lightwalletd => [
                127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 62, 0, 1, 0, 0, 0, 64,
                188, 71, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 56, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 64, 0, 56, 0, 9, 0, 64, 0, 36, 0, 33, 0,
            ],
            TestingBinaryVariant::Zainod => [
                127, 69, 76, 70, 2, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 0,
                87, 19, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 112, 143, 238, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 64, 0, 56, 0, 14, 0, 64, 0, 34, 0, 33, 0,
            ],
            TestingBinaryVariant::Zcashd => [
                127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 0,
                58, 121, 3, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 8, 39, 154, 10, 0, 0, 0, 0, 0, 0,
                0, 0, 64, 0, 56, 0, 12, 0, 64, 0, 47, 0, 45, 0,
            ],
            TestingBinaryVariant::ZcashCli => [
                127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 208,
                254, 85, 3, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 216, 43, 87, 4, 0, 0, 0, 0, 0, 0,
                0, 0, 64, 0, 56, 0, 12, 0, 64, 0, 47, 0, 45, 0,
            ],
            TestingBinaryVariant::Zebrad => [
                127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 48,
                151, 16, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 56, 16, 122, 4, 0, 0, 0, 0, 0, 0,
                0, 0, 64, 0, 56, 0, 14, 0, 64, 0, 34, 0, 33, 0,
            ],
            TestingBinaryVariant::ZingoCli => [
                127, 69, 76, 70, 2, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 62, 0, 1, 0, 0, 0, 208,
                141, 33, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 152, 215, 66, 5, 0, 0, 0, 0, 0, 0,
                0, 0, 64, 0, 56, 0, 14, 0, 64, 0, 34, 0, 33, 0,
            ],
        }
    }

    /// TODO! fix this to use correct dirs
    /// Returns the get version string of this [`TestingBinary`].
    pub fn get_version_string(&self) -> String {
        let s = match &self.variant {
            TestingBinaryVariant::Lightwalletd => {
                "Use \"lightwalletd [command] --help\" for more information about a command."
            }
            TestingBinaryVariant::Zainod => "zainod [OPTIONS]",
            TestingBinaryVariant::ZcashCli => "Zcash RPC client version v6.1.0",
            TestingBinaryVariant::Zcashd => "Zcash Daemon version v6.1.0",
            TestingBinaryVariant::Zebrad => "zebrad 2.1.0",
            TestingBinaryVariant::ZingoCli => "Zingo CLI 0.1.1",
        };
        s.to_string()
    }

    /// Checksum for each binary
    pub fn get_shasum(&self) -> String {
        match &self.variant {
            TestingBinaryVariant::Lightwalletd => todo!(),
            TestingBinaryVariant::Zainod => todo!(),
            TestingBinaryVariant::ZcashCli => todo!(),
            TestingBinaryVariant::Zcashd => todo!(),
            TestingBinaryVariant::Zebrad => todo!(),
            TestingBinaryVariant::ZingoCli => todo!(),
        }
    }

    // pub fn get_version_string(&self) -> String {}
    async fn validate(&self) {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("cargo manifest path to be found");
        let bin_dir = Path::new(&crate_dir)
            .join("fetched_resources/test_binaries")
            .join(&self.get_name());
        let bin_path = bin_dir.join(&self.get_name());
        let shasum_path = bin_dir.join("shasum");

        loop {
            if !bin_path.is_file() {
                println!("{:?} = file not found!", &bin_path);
                fetch_binary(&bin_path, &self.name).await;
            }

            if bin_path.is_file() {
                match self.confirm(&bin_path, &shasum_path).await {
                    Ok(()) => {
                        println!("{} binary confirmed.", &self.name);
                        break;
                    }
                    Err(_) => {
                        println!("Binary confirmation failure, deleted found binary. Please fetch again.");
                        fs::remove_file(&bin_path).expect("Failed to delete binary");
                    }
                }
            }
        }
    }

    async fn confirm(&self, bin_path: &PathBuf, shasum_path: &PathBuf) -> Result<(), ()> {
        let file_read_sample = File::open(bin_path).expect("File to be readable");
        let mut reader = BufReader::with_capacity(64, file_read_sample);
        let bytes_read = reader.fill_buf().expect("Reader to fill_buf").to_vec();

        if bytes_read != self.byte_pattern {
            fs::remove_file(bin_path).expect("Binary to be deleted");
            println!("Binary {} removed!", self.name);
            return Err(());
        }

        // Check binary version
        let mut output = Command::new(bin_path)
            .arg("--version")
            .output()
            .expect("Failed to execute command");

        if !String::from_utf8_lossy(&output.stderr).contains(&self.version_string) {
            fs::remove_file(bin_path).expect("Binary to be deleted on version check fail");
            println!("Version string for {} is incorrect", self.name);
            return Err(());
        }

        // Check hash
        let hash = sha512sum_file(bin_path);
        let mut buf = BufReader::new(File::open(shasum_path).expect("Shasum file to open"));
        let mut shasum_record = String::new();
        buf.read_to_string(&mut shasum_record)
            .expect("Buffer to write into String");

        if shasum_record.contains(&self.name) {
            let hash_record = shasum_record
                .split_whitespace()
                .next()
                .expect("Shasum record to be splittable");
            if hash != hash_record {
                fs::remove_file(bin_path).expect("Binary to be deleted on hash mismatch");
                return Err(());
            }
        }

        Ok(())
    }
}
