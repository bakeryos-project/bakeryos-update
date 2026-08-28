use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn get_update_avaliable_package() -> Result<Vec<String>, String> {
    let output = Command::new("checkupdates").output();

    let output = match output {
        Ok(out) if out.status.success() => out,
        _ => {
            let status = Command::new("pkexec")
                .arg("pacman")
                .arg("-Sy")
                .status()
                .map_err(|e| format!("Error: {}", e))?;

            if !status.success() {
                return Err("The user cancelled or root authentication failed.!".to_string());
            }

            Command::new("pacman")
                .arg("-Qu")
                .output()
                .map_err(|e| format!("Error: {}", e))?
        }
    };

    let stdout_str =
        String::from_utf8(output.stdout).map_err(|e| format!("UTF-8 decoding error: {}", e))?;

    let mut package_names = Vec::new();

    for line in stdout_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(pkg_name) = parts.first() {
            package_names.push(pkg_name.to_string());
        }
    }

    Ok(package_names)
}

pub fn update_package(packages: Vec<String>) -> Result<(), String> {
    if packages.is_empty() {
        return Ok(());
    }

    let mut child = Command::new("pkexec")
        .arg("pacman")
        .arg("-Syu")
        .arg("--noconfirm")
        .args(&packages)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    println!("[Pacman Out]: {}", content);
                }
                Err(e) => {
                    eprintln!("Error reading stdout: {}", e);
                }
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(content) = line {
                eprintln!("[Pacman Err]: {}", content);
            }
        }
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait on process: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Pacman exited with non-zero status: {}", status))
    }
}
