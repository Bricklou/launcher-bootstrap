use std::{
    fs::{create_dir_all, remove_file, File},
    io::{Error, ErrorKind, Read, Result, Write},
    os::unix::net::{UnixListener, UnixStream},
    process::Command,
};
use tauri::api::path::data_dir;
use tracing;

use super::ID;

pub fn register<F: FnMut(String) + Send + 'static>(scheme: &str, handler: F) -> Result<()> {
    listen(handler);

    let mut target = data_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "data directory not found"))?
        .join("applications");

    create_dir_all(&target)?;

    let exe = tauri::utils::platform::current_exe()?;

    let file_name = format!(
        "{}-handler.desktop",
        exe.file_name()
            .ok_or_else(|| Error::new(
                ErrorKind::NotFound,
                "Couldn't get file name of current executable."
            ))?
            .to_string_lossy()
    );

    target.push(&file_name);

    let mime_types = format!("x-scheme-handler/{}", scheme);

    let mut file = File::create(&target)?;
    file.write_all(
        format!(
            include_str!("template.desktop"),
            name = ID
                .get()
                .expect("Called register() before prepare()")
                .split('.')
                .last()
                .unwrap(),
            exec = exe.to_string_lossy(),
            mime_types = mime_types
        )
        .as_bytes(),
    )?;

    target.pop();

    Command::new("update-desktop-database")
        .arg(target)
        .status()?;

    Command::new("xdg-mime")
        .args(["default", &file_name, scheme])
        .status()?;

    Ok(())
}

pub fn listen<F: FnMut(String) + Send + 'static>(mut handler: F) {
    std::thread::spawn(move || {
        let addr = format!(
            "/tmp/{}-deep-link.sock",
            ID.get().expect("listen() called before prepare()")
        );

        let listener = UnixListener::bind(addr).expect("Can't create listener");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    if let Err(io_error) = stream.read_to_string(&mut buffer) {
                        tracing::error!(
                            "Error reading incoming connection: {}",
                            io_error.to_string()
                        );
                    };

                    handler(dbg!(buffer));
                }
                Err(err) => {
                    tracing::error!("Incoming connection failed: {}", err);
                    continue;
                }
            }
        }
    });
}

pub fn prepare(identifier: &str) {
    let addr = format!("/tmp/{}-deep-link.sock", identifier);

    match UnixStream::connect(&addr) {
        Ok(mut stream) => {
            if let Err(io_err) =
                stream.write_all(std::env::args().nth(1).unwrap_or_default().as_bytes())
            {
                tracing::error!(
                    "Error sending message to primary instance: {}",
                    io_err.to_string()
                );
            };
            std::process::exit(0);
        }
        Err(err) => {
            tracing::error!("Error creating socket listener: {}", err.to_string());
            if err.kind() == ErrorKind::ConnectionRefused {
                let _ = remove_file(&addr);
            }
        }
    };
    ID.set(identifier.to_string())
        .expect("prepare() called more than once with different identifiers.");
}
