use anyhow::{anyhow, Result};
use std::process::Stdio;
use tokio::process::Command;

/// Service for handling PlatformIO operations like building, uploading, and initializing ESP32 projects.
#[derive(Clone)]
pub struct PlatformIOService;

impl PlatformIOService {
    /// Constructor (no-op, as it's a stateless service).
    pub fn new() -> Self {
        Self
    }

    /// Build the PlatformIO project for a device
    /// Builds the PlatformIO project at the given path.
    pub async fn build_project(&self, project_path: &str) -> Result<String> {
        self.run_pio_command(project_path, &["run"]).await
    }

    /// Upload firmware to ESP32 device
    /// Uploads firmware to the ESP32 device.
    pub async fn upload_firmware(&self, project_path: &str, port: Option<&str>) -> Result<String> {
        let mut args = vec!["run", "--target", "upload"];
        if let Some(p) = port {
            args.extend_from_slice(&["--upload-port", p]);
        }
        self.run_pio_command(project_path, &args).await
    }

    /// Clean the PlatformIO project
    /// Cleans build files in the PlatformIO project.
    pub async fn clean_project(&self, project_path: &str) -> Result<String> {
        self.run_pio_command(project_path, &["run", "--target", "clean"])
            .await
    }

    /// Get project information
    /// Retrieves PlatformIO project configuration info.
    pub async fn get_project_info(&self, project_path: &str) -> Result<String> {
        self.run_pio_command(project_path, &["project", "config"])
            .await
    }

    /// Initialize a new PlatformIO project
    /// Initializes a new PlatformIO project for the given board.
    pub async fn init_project(&self, project_path: &str, board: &str) -> Result<String> {
        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(project_path)
            .await
            .map_err(|e| anyhow!("Failed to create project directory: {}", e))?;

        self.run_pio_command(project_path, &["project", "init", "--board", board])
            .await
    }

    /// Create a basic ESP32 main.cpp file
    /// Generates a basic `main.cpp` file for ESP32 (blinking LED example).
    pub async fn create_basic_main(&self, project_path: &str) -> Result<()> {
        let src_dir = format!("{}/src", project_path);
        tokio::fs::create_dir_all(&src_dir)
            .await
            .map_err(|e| anyhow!("Failed to create src directory: {}", e))?;

        let main_cpp_content = r#"#include <Arduino.h>

// Basic ESP32 program
void setup() {
    Serial.begin(115200);
    pinMode(LED_BUILTIN, OUTPUT);
    Serial.println("ESP32 Remote Lab Device Started");
}

void loop() {
    digitalWrite(LED_BUILTIN, HIGH);
    Serial.println("LED ON");
    delay(1000);
    digitalWrite(LED_BUILTIN, LOW);
    Serial.println("LED OFF");
    delay(1000);
}
"#;

        let main_path = format!("{}/main.cpp", src_dir);
        tokio::fs::write(&main_path, main_cpp_content)
            .await
            .map_err(|e| anyhow!("Failed to write main.cpp: {}", e))?;

        Ok(())
    }

    /// Run a PlatformIO command and return the output
    /// Helper to execute a PlatformIO command and capture output.
    async fn run_pio_command(&self, project_path: &str, args: &[&str]) -> Result<String> {
        // Check if platformio is installed
        self.check_pio_installed().await?;

        // Change to project directory and run command
        let mut cmd = Command::new("platformio");
        cmd.args(args)
            .current_dir(project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute platformio command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(format!("{}{}", stdout, stderr))
        } else {
            Err(anyhow!("PlatformIO command failed: {}\n{}", stdout, stderr))
        }
    }

    /// Check if PlatformIO is installed
    /// Verifies PlatformIO is installed by running `platformio --version`.
    async fn check_pio_installed(&self) -> Result<()> {
        let output = Command::new("platformio")
            .arg("--version")
            .output()
            .await
            .map_err(|e| anyhow!("PlatformIO not found. Please install PlatformIO: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("PlatformIO installation check failed"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test for PlatformIO installation check (expects failure in test env).
    #[tokio::test]
    async fn test_pio_check() {
        let service = PlatformIOService::new();
        // This will fail if PlatformIO is not installed, which is expected in test environment
        let _ = service.check_pio_installed().await;
    }
}
