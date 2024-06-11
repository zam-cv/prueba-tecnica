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
        cmd!("git", "clone", "https://github.com/zam-cv/prueba-tecnica").run()?;
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
    if let Err(_) = which("choco") {
        cmd!("powershell", "-Command", "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))")
            .run()?;
    }

    // Install Docker Desktop
    cmd!("choco", "install", "docker-desktop", "-y").run()?;

    // Install Docker Compose
    cmd!("choco", "install", "docker-compose", "-y").run()?;

    // Install Git
    cmd!("choco", "install", "git", "-y").run()?;

    Ok(())
}

#[allow(dead_code)]
fn install_on_linux() -> anyhow::Result<()> {
    cmd!("sudo", "apt-get", "update").run()?;

    // Install Docker
    cmd!("sudo", "apt-get", "install", "docker.io", "-y").run()?;
    cmd!("sudo", "systemctl", "start", "docker").run()?;
    cmd!("sudo", "systemctl", "enable", "docker").run()?;
    cmd!("sudo", "usermod", "-aG", "docker", "$USER").run()?;
    cmd!("newgrp", "docker").run()?;

    // Install Docker Compose
    cmd!("sudo", "apt-get", "install", "docker-compose", "-y").run()?;
    cmd!(
        "sudo",
        "ln",
        "-s",
        "/usr/bin/docker-compose",
        "/usr/local/bin/docker-compose"
    )
    .run()?;
    cmd!("sudo", "chmod", "+x", "/usr/local/bin/docker-compose").run()?;

    // Install Git
    cmd!("sudo", "apt-get", "install", "git", "-y").run()?;

    Ok(())
}

#[allow(dead_code)]
fn install_on_macos() -> anyhow::Result<()> {
    // Install Homebrew
    if let Err(_) = which("brew") {
        cmd!(
            "/bin/bash",
            "-c",
            "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        )
        .run()?;

        cmd!("brew", "update").run()?;
    }

    // Install Docker Desktop
    cmd!("brew", "install", "docker").run()?;

    // Install Docker Compose
    cmd!("brew", "install", "docker-compose").run()?;

    // Install Git
    cmd!("brew", "install", "git").run()?;

    Ok(())
}
