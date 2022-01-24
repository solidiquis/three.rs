use command_group::{CommandGroup, GroupChild};
use std::cell::Cell;
use std::io::{BufReader, BufRead, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;

pub struct WebpackDevServer {
    process: Cell<Option<GroupChild>>,
    pid: Cell<Option<u32>>,
    compile_timeout_in_sec: Cell<u8>
}

impl Default for WebpackDevServer {
    fn default() -> Self {
        Self {
            process: Cell::new(None),
            pid: Cell::new(None),
            compile_timeout_in_sec: Cell::new(10)
        }
    }
}

impl WebpackDevServer {
    pub fn init(&self) -> Result<(), String> {
        let mut webpack_dev_server = Command::new("npm")
            .arg("run")
            .arg("dev")
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .group_spawn()
            .or_else(|err| Err(format!["Failed to spawn webpack dev server with err: {}", err]))?;

        let mut webpack_stdout = BufReader::new(
            webpack_dev_server.inner().stdout.take().unwrap()
        );

        let mut webpack_stderr = BufReader::new(
            webpack_dev_server.inner().stderr.take().unwrap()
        );

        let (stdin_tx, stdin_rx) = channel();
        let (stderr_tx, stderr_rx) = channel();

        let stdin_ln = thread::spawn(move || {
            loop {
                let mut line = String::new();

                webpack_stdout.read_line(&mut line).unwrap();

                if line.contains("webpack") && line.contains("compiled successfully") {
                    stdin_tx.send(0).unwrap();
                    return;
                }
            }
        });

        let stderr_ln = thread::spawn(move || {
            let mut buf_str = "".to_string();

            webpack_stderr.read_to_string(&mut buf_str).unwrap();

            stderr_tx.send(buf_str).unwrap();
        });

        let start = Instant::now();

        loop {
            if Instant::now().duration_since(start).as_secs() > 10 {
                drop(stderr_ln);
                drop(stdin_ln);
                return Err("Webpack took too long to compile assets.".to_string());
            }

            match stdin_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(_) => break,
                _ => ()
            }

            match stderr_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(e) => return Err(format!["Webpack failed to start with err: {}.", e]),
                _ => ()
            }
        } 

        self.pid.set(Some(webpack_dev_server.id()));
        self.process.set(Some(webpack_dev_server));

        Ok(())
    }

    pub fn kill(&self) {
        match self.process.replace(None) {
            Some(mut p) => p.kill().unwrap_or(()),
            None => ()
        };
    }

    pub fn set_timeout_in_secs(&self, timeout_in_sec: u8) {
        self.compile_timeout_in_sec.set(timeout_in_sec);
    }
}
