use eyre::bail;
use eyre::Result;
use ssh2::Session;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

// Based on https://github.com/Srynetix/pssh-rs

/// SSH command output.
pub struct SshOutput {
    /// Exit status
    pub exit_status: i32,
    /// Standard error
    pub stderr: Vec<u8>,
    /// Standard output
    pub stdout: Vec<u8>,
}

pub struct SshClient {
    addr: SocketAddr,
    pass: String,
    session: Option<Session>,
    timeout: u64,
    user: String,
}

impl SshClient {
    pub fn from(
        user: impl Into<String>,
        addr: impl Into<SocketAddr>,
        pass: impl Into<String>,
    ) -> Self {
        Self {
            addr: addr.into(),
            pass: pass.into(),
            session: None,
            timeout: 0,
            user: user.into(),
        }
    }

    pub fn execute(&mut self, command: &str) -> Result<SshOutput> {
        print!("running remote command: {}", command);

        // Establish authenticated SSH session.
        if self.session.is_none() {
            self.connect()?;
        }
        let session = self.session.as_ref().unwrap();

        // Open channel and stderr stream.
        let mut channel = session.channel_session()?;
        let mut stderr_stream = channel.stderr();

        // Execute command.
        channel.exec(command)?;

        // Read stdout into buffer.
        let mut stdout = Vec::new();
        channel.read_to_end(&mut stdout)?;

        // Read stderr into buffer.
        let mut stderr = Vec::new();
        stderr_stream.read_to_end(&mut stderr)?;

        // Close channel and retrieve exit status.
        channel.wait_close()?;
        let exit_status = channel.exit_status()?;

        println!("exit code: {}", exit_status);
        println!("   stdout: {}", String::from_utf8(stdout.clone())?);
        println!("   stderr: {}", String::from_utf8(stderr.clone())?);

        // Return successfully.
        Ok(SshOutput {
            exit_status,
            stdout,
            stderr,
        })
    }

    pub fn scp_download<P: AsRef<Path>>(&mut self, remote_path: P, local_path: P) -> Result<()> {
        // Establish authenticated SSH session.
        if self.session.is_none() {
            self.connect()?;
        }
        let session = self.session.as_ref().unwrap();

        // Open channel.
        let (mut channel, _) = session.scp_recv(remote_path.as_ref())?;

        // Read remote file into buffer.
        let mut buffer = Vec::new();
        channel.read_to_end(&mut buffer)?;

        // Write buffer to local file.
        std::fs::write(local_path, &buffer)?;

        // Close channel.
        channel.send_eof()?;
        channel.wait_eof()?;
        channel.close()?;
        channel.wait_close()?;

        // Return successfully.
        Ok(())
    }

    pub fn scp_upload<P: AsRef<Path>, Q: AsRef<Path>>(
        &mut self,
        local_path: P,
        remote_path: Q,
    ) -> Result<()> {
        // Establish authenticated SSH session.
        if self.session.is_none() {
            self.connect()?;
        }
        let session = self.session.as_ref().unwrap();

        // Read local file into buffer.
        let buffer = std::fs::read(local_path)?;
        let size = buffer.len() as u64;

        // Open channel.
        let mut channel = session.scp_send(remote_path.as_ref(), 0o644, size, None)?;

        // Write buffer to remote file.
        channel.write_all(&buffer)?;

        // Close channel.
        channel.send_eof()?;
        channel.wait_eof()?;
        channel.close()?;
        channel.wait_close()?;

        // Return successfully.
        Ok(())
    }

    pub fn connect(&mut self) -> Result<&mut Self> {
        print!("Connecting to: {}", self.addr);

        // Initialize new SSH session.
        let mut session = Session::new()?;

        // Open a TCP connection to the configured host and attach it to the SSH session.
        let tcp_stream = if self.timeout == 0 {
            // If timeout is zero, don't set a timeout.
            TcpStream::connect(&self.addr)?
        } else {
            // If timeout is non-zero, set a timeout on both the SSH session and the TCP stream.
            session.set_timeout(self.timeout as u32);
            TcpStream::connect_timeout(&self.addr, Duration::from_millis(self.timeout))?
        };
        session.set_tcp_stream(tcp_stream);

        // Perform SSH handshake.
        session.handshake()?;

        session.userauth_password(&self.user, &self.pass)?;

        // Confirm that the session is authenticated.
        if !session.authenticated() {
            bail!("Authentication failed");
        }

        // Cache authenticated session and return successfully.
        self.session = Some(session);
        Ok(self)
    }

    pub fn disconnect(&mut self) -> &mut Self {
        self.session = None;
        self
    }
}
