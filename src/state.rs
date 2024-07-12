use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio;


pub struct State {
    state_fp: &'static Path,
    matched_http_status: String,
}

impl State {
    pub fn new() -> Self {
        State {
            state_fp: Path::new("state"),
            matched_http_status: String::from("false"),
        }
    }

    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        if !self.state_fp.exists() {
            let mut state_file = File::create(&self.state_fp)
                .map_err(|e| format!("Failed to create file: {}", e))?;

            state_file.write_all(self.matched_http_status.as_bytes())
                .map_err(|e| format!("Failed to write into file: {}", e))?;
        }
        Ok(())
    }

    pub async fn write_state(&self, state: &str) {
        let _ = tokio::fs::write(self.state_fp, &state.as_bytes()).await;
    }

    pub async fn read_state(&self) -> String {
        let content = tokio::fs::read_to_string(self.state_fp).await
            .unwrap()
            .trim()
            .to_lowercase()
            .to_string();
        content
    }
}
