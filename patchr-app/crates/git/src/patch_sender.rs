use std::{fs, io, mem::MaybeUninit, path::Path, process, ptr};

use common::{
    constants::GIT_COMMAND,
    util::{git::commit_hash_valid, rust::to_unit},
};
use email_address::EmailAddress;
use uuid::Uuid;

use crate::{series::Series, GitError, GitErrorCode};

// Constants for text replacement
// We do this because we want to keep the series manager as transparent as possible
// Mailing lists receiving series from a patchr user should not have to adapt
pub const COVER_LETTER_FILE_NAME: &str = "0000-cover-letter.patch";
pub const CV_PATCH_SUBJECT_PLACEHOLDER: &str = "*** SUBJECT HERE ***";
pub const CV_PATCH_DESCRIPTION_PLACEHOLDER: &str = "*** BLURB HERE ***";

// TODO: maybe we should use a builder if the number of arguments increases again
pub trait PatchSender {
    fn send(
        &self, series: &Series, to_email: &str, output_dir: &Path, first_commit: &str,
        last_commit: &str, cc: Option<&str>,
    ) -> Result<(), GitError>;
}

pub struct GitPatchSender<'a> {
    from_email: &'a str,
    send_command: Option<&'a str>,
    smtp_server: Option<&'a str>,
    smtp_port: Option<u16>,
    smtp_user: Option<&'a str>,
    smtp_encryption: Option<&'a str>,
}

pub struct GitPatchSenderBuilder<'a> {
    from_email: &'a str,
    send_command: Option<&'a str>,
    smtp_server: Option<&'a str>,
    smtp_port: Option<u16>,
    smtp_user: Option<&'a str>,
    smtp_encryption: Option<&'a str>,
}

impl<'a> GitPatchSenderBuilder<'a> {
    fn new(from_email: &'a str) -> Self {
        Self {
            from_email,
            send_command: None,
            smtp_server: None,
            smtp_port: None,
            smtp_user: None,
            smtp_encryption: None,
        }
    }

    pub fn set_send_command(&mut self, send_command: &'a str) {
        self.send_command = Some(send_command);
    }

    pub fn set_smtp_server(&mut self, smtp_server: &'a str) {
        self.smtp_server = Some(smtp_server);
    }

    pub fn set_smtp_port(&mut self, smtp_port: u16) {
        self.smtp_port = Some(smtp_port);
    }

    pub fn set_smtp_user(&mut self, smtp_user: &'a str) {
        self.smtp_user = Some(smtp_user);
    }

    pub fn set_smtp_encryption(&mut self, smtp_encryption: &'a str) {
        self.smtp_encryption = Some(smtp_encryption)
    }

    pub fn build(&self) -> GitPatchSender {
        GitPatchSender::new(
            self.from_email,
            self.send_command,
            self.smtp_server,
            self.smtp_port,
            self.smtp_user,
            self.smtp_encryption,
        )
    }
}

impl<'a> GitPatchSender<'a> {
    fn new(
        from_email: &'a str, send_command: Option<&'a str>, smtp_server: Option<&'a str>,
        smtp_port: Option<u16>, smtp_user: Option<&'a str>, smtp_encryption: Option<&'a str>,
    ) -> Self {
        Self {
            send_command,
            from_email,
            smtp_server,
            smtp_port,
            smtp_user,
            smtp_encryption,
        }
    }

    pub fn builder(from_email: &'a str) -> GitPatchSenderBuilder {
        GitPatchSenderBuilder::new(from_email)
    }

    fn setup_signal_handler(signal: i32, handler: usize) {
        unsafe {
            let mut mask: libc::sigset_t = MaybeUninit::zeroed().assume_init();
            if libc::sigemptyset(&mut mask as *mut libc::sigset_t) != 0 {
                panic!("Failed to init sigset_t");
            }
            let sigaction = libc::sigaction {
                sa_sigaction: handler,
                sa_flags: 0,
                sa_mask: mask,
                sa_restorer: None,
            };
            if libc::sigaction(signal, &sigaction as *const libc::sigaction, ptr::null_mut()) != 0 {
                panic!("Failed to init signal handler");
            }
        }
    }
}

impl PatchSender for GitPatchSender<'_> {
    fn send(
        &self, series: &Series, to_email: &str, output_dir: &Path, first_commit: &str,
        last_commit: &str, cc: Option<&str>,
    ) -> Result<(), GitError> {
        if !fs::metadata(output_dir).is_ok_and(|m| m.is_dir()) {
            return Err(GitError::new(
                crate::GitErrorCode::InvalidPath,
                String::from("Invalid output path"),
            ));
        }
        if !commit_hash_valid(first_commit) || !commit_hash_valid(last_commit) {
            return Err(GitError::new(
                crate::GitErrorCode::StringFormatError,
                String::from("Invalid commit hash format"),
            ));
        }
        if !EmailAddress::is_valid(self.from_email) || !EmailAddress::is_valid(to_email) {
            return Err(GitError::new(
                crate::GitErrorCode::StringFormatError,
                String::from("Invalid email address"),
            ));
        }

        let tmp_out = output_dir.join(Uuid::new_v4().to_string());

        let return_err = |e: io::Error| {
            Err::<(), GitError>(GitError::new(GitErrorCode::SendSeriesFailed, e.to_string()))
        };
        let clean_and_return_err = |e: io::Error| {
            let _ = fs::remove_dir_all(&tmp_out);
            return_err(e)
        };

        fs::create_dir(&tmp_out).or_else(return_err)?;

        let mut short_name = String::from(series.short_name());
        if !short_name.is_empty() {
            short_name.push(' ');
        }
        process::Command::new(GIT_COMMAND)
            .arg("format-patch")
            .arg("--cover-letter")
            .arg("-n") // numbered
            .arg("-o")
            .arg(&tmp_out) // output
            .arg(format!(
                "--subject-prefix=PATCH {}v{}",
                short_name,
                series.current_revision()
            ))
            .arg(format!("{}..{}", first_commit, last_commit))
            .status()
            .map(to_unit)
            .or_else(clean_and_return_err)?;

        // prepare cover letter
        let cv_path = tmp_out.join(COVER_LETTER_FILE_NAME);
        let read_cv_res = fs::read_to_string(&cv_path);
        let Ok(cv_content) = read_cv_res else {
            return clean_and_return_err(read_cv_res.err().unwrap());
        };
        let cv_content = cv_content
            .replace(CV_PATCH_SUBJECT_PLACEHOLDER, series.title())
            .replace(CV_PATCH_DESCRIPTION_PLACEHOLDER, series.to_string().as_str());
        fs::write(&cv_path, cv_content).or_else(clean_and_return_err)?;

        let mut send_email_cmd = process::Command::new(GIT_COMMAND);
        send_email_cmd
            .arg("send-email")
            .arg(format!("--from={}", self.from_email))
            .arg(format!("--to={}", to_email))
            .arg(&tmp_out);

        if let Some(cmd) = self.send_command {
            send_email_cmd.arg(format!("--sendmail-cmd={}", cmd));
        };
        if let Some(user) = self.smtp_user {
            send_email_cmd.arg(format!("--smtp-user={}", user));
        };
        if let Some(server) = self.smtp_server {
            send_email_cmd.arg(format!("--smtp-server={}", server));
        };
        if let Some(port) = self.smtp_port {
            send_email_cmd.arg(format!("--smtp-server-port={}", port));
        };
        if let Some(encryption) = self.smtp_encryption {
            send_email_cmd.arg(format!("--smtp-encryption={}", encryption));
        };
        if let Some(c) = cc {
            send_email_cmd.arg(format!("--cc={}", c));
        };

        // ignore Ctrl+C
        Self::setup_signal_handler(libc::SIGINT, libc::SIG_IGN);
        let send_email_cmd_res = send_email_cmd.status();
        Self::setup_signal_handler(libc::SIGINT, libc::SIG_DFL);

        let _ = fs::remove_dir_all(&tmp_out);
        let send_email_cmd_res = send_email_cmd_res
            .map_err(|e| GitError::new(GitErrorCode::CommandExecutionFailed, e.to_string()));
        match send_email_cmd_res {
            Ok(res) => {
                if res.success() {
                    Ok(())
                } else {
                    Err(GitError::new(
                        GitErrorCode::CommandExecutionFailed,
                        String::from("subcommand failed"),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
