use cfg_if::cfg_if;
use dirs::home_dir;
use duct::cmd;
use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use which::which;

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Desktop directory not found"))?;
    env::set_current_dir(&home)?;

    cfg_if! {
        if #[cfg(target_os = "windows")] {
            install_on_windows()?;
        } else if #[cfg(target_os = "linux")] {
            install_on_linux()?;
        } else if #[cfg(target_os = "macos")] {
            install_on_macos()?;
        } else {
            println!("Unsupported OS")
        }
    }

    if !home.join("prueba-tecnica").exists() {
        let _ = cmd!("git", "clone", "https://github.com/zam-cv/prueba-tecnica").run();
    }

    env::set_current_dir(home.join("prueba-tecnica"))?;
    cmd!("docker-compose", "--profile", "prod", "up").run()?;

    // Open the browser
    open::that("http://localhost:8080")?;

    println!("Press Ctrl+C to stop the server.");
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Cleanup
    println!("Cleaning up...");
    cmd!("docker-compose", "down").run()?;
    println!("Cleanup complete, exiting now.");

    Ok(())
}

#[allow(dead_code)]
fn install_on_windows() -> anyhow::Result<()> {
    // Install Chocolatey
    if which("choco").is_err() {
        cmd!(
            "powershell",
            "-Command",
            "Set-ExecutionPolicy Bypass -Scope Process -Force"
        )
        .run()?;

        if std::fs::metadata("C:\\ProgramData\\chocolatey").is_ok() {
            cmd!("rmdir", "/s", "/q", "C:\\ProgramData\\chocolatey").run()?;
        }

        cmd!("powershell", "-Command", "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))")
            .run()?;
    }

    // Install Docker Desktop
    if which("docker").is_err() {
        cmd!("choco", "install", "docker-desktop", "-y").run()?;
    }

    println!("Loading Docker Desktop, please wait...");
    let _ = cmd!("start", "docker-desktop").run();
    std::thread::sleep(std::time::Duration::from_secs(60));

    // Install Docker Compose
    if which("docker-compose").is_err() {
        let _ = cmd!("choco", "install", "docker-compose", "-y").run();
    }

    // Install Git
    if which("git").is_err() {
        let _ = cmd!("choco", "install", "git", "-y").run();
    }

    Ok(())
}

#[allow(dead_code)]
fn install_on_linux() -> anyhow::Result<()> {
    let _ = cmd!("sudo", "apt-get", "update").run();

    // Install Docker
    if which("docker").is_err() {
        cmd!("sudo", "apt-get", "install", "docker.io", "-y").run()?;
    }

    // Install Docker Compose
    if which("docker-compose").is_err() {
        cmd!("sudo", "apt-get", "install", "docker-compose", "-y").run()?;
    }

    // Install Git
    if which("git").is_err() {
        cmd!("sudo", "apt-get", "install", "git", "-y").run()?;
    }

    Ok(())
}

#[allow(dead_code)]
fn install_on_macos() -> anyhow::Result<()> {
    // Install Homebrew
    if which("brew").is_err() {
        let _ = cmd!(
            "/bin/bash",
            "-c",
            "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        )
        .run();

        let _ = cmd!("brew", "update").run();
    }

    if which("docker").is_err() {
        let _ = cmd!("brew", "install", "--cask", "docker").run();
    }
    
    println!("Loading Docker Desktop, please wait...");
    let _ = cmd!("open", "/Applications/Docker.app").run();
    std::thread::sleep(std::time::Duration::from_secs(60));

    // Install Docker Compose
    if which("docker-compose").is_err() {
        let _ = cmd!("brew", "install", "docker-compose").run();
    }

    // Install Git
    if which("git").is_err() {
        let _ = cmd!("brew", "install", "git").run();
    }

    Ok(())
}
