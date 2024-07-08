use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::ControlFlow,
};

use common::util::{input::sanitize_cc_list, misc::{DEFAULT_DATE_TIME_FORMAT, LINE_SEP}};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{GitError, GitErrorCode};

lazy_static! {
    static ref SERIES_TITLE_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9 _-]+$").unwrap();
    static ref SERIES_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    static ref SERIES_SHORT_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{1,8}$").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Series {
    name: String,
    title: String,
    cover_letter: String,
    short_name: String,
    revisions: Vec<SeriesRevision>,
    logs: RefCell<Vec<SeriesLog>>,
    cc: String,
}

// We only store the content for
// now but we may add fields in the future
#[derive(Serialize, Deserialize)]
pub struct SeriesRevision {
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct SeriesLog(String);

impl SeriesLog {
    fn log(series: &Series, op: &str, msg: &str) {
        let log = SeriesLog(format!(
            "{}: {} - {}",
            chrono::offset::Local::now().format(DEFAULT_DATE_TIME_FORMAT),
            op,
            msg
        ));
        series.log(log);
    }

    pub fn send(series: &Series, to: &str) {
        Self::log(
            series,
            "send",
            &format!("revision {} send to {}", series.current_revision(), to),
        );
    }

    pub fn add_revision(series: &mut Series, rev: usize) {
        Self::log(series, "add revision", &format!("revision updated to {}", rev));
    }

    pub fn delete_revision(series: &mut Series, rev: usize) {
        Self::log(series, "delete revision", &format!("revision {} deleted", rev));
    }
}

impl Series {
    pub fn new(name: &str, title: &str) -> Option<Self> {
        let (Some(title), Some(name)) =
            (Series::validate_title(title), Series::validate_name(name))
        else {
            return None;
        };

        Some(Self {
            name: String::from(name),
            title: String::from(title),
            cover_letter: String::new(),
            short_name: String::new(),
            revisions: Vec::new(),
            logs: RefCell::new(Vec::new()),
            cc: String::new(),
        })
    }

    pub fn current_revision(&self) -> u32 {
        (self.revisions.len() + 1) as u32
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), GitError> {
        if let Some(name) = Series::validate_name(name) {
            self.name = String::from(name);
            Ok(())
        } else {
            Err(GitError::new(
                GitErrorCode::StringFormatError,
                String::from("Invalid name format"),
            ))
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), GitError> {
        if let Some(title) = Series::validate_title(title) {
            self.title = String::from(title);
            Ok(())
        } else {
            Err(GitError::new(
                GitErrorCode::StringFormatError,
                String::from("Invalid title format"),
            ))
        }
    }

    pub fn cover_letter(&self) -> &str {
        self.cover_letter.as_str()
    }

    pub fn set_cover_letter(&mut self, cover_letter: &str) -> Result<(), GitError> {
        self.cover_letter = String::from(cover_letter.trim());
        Ok(())
    }

    pub fn short_name(&self) -> &str {
        self.short_name.as_str()
    }

    pub fn set_short_name(&mut self, short_name: &str) -> Result<(), GitError> {
        if let Some(short_name) = Series::validate_short_name(short_name) {
            self.short_name = String::from(short_name);
            Ok(())
        } else {
            Err(GitError::new(
                GitErrorCode::StringFormatError,
                String::from("Invalid short name format"),
            ))
        }
    }

    pub fn cc(&self) -> &str {
        self.cc.as_str()
    }

    pub fn set_cc(&mut self, cc: &str) -> Result<(), GitError> {
        if let Some(sanitized_cc) = sanitize_cc_list(cc) {
            self.cc = String::from(sanitized_cc);
            Ok(())
        } else {
            Err(GitError::new(
                GitErrorCode::StringFormatError,
                String::from("Invalid cc list format"),
            ))
        }
    }

    pub fn add_revision(&mut self) {
        self.revisions.push(SeriesRevision::new(""));
        SeriesLog::add_revision(self, self.current_revision() as usize);
    }

    fn revision_index(rev: usize) -> Option<usize> {
        if rev < 2 {
            None
        } else {
            Some(rev - 2)
        }
    }

    pub fn delete_revision(&mut self, n: usize) {
        let Some(i) = Self::revision_index(n) else {
            return;
        };
        if i < self.revisions.len() {
            self.revisions.remove(i);
            SeriesLog::delete_revision(self, n);
        }
    }

    pub fn revision_mut(&mut self, n: usize) -> Option<&mut SeriesRevision> {
        let Some(n) = Self::revision_index(n) else {
            return None;
        };
        self.revisions.get_mut(n)
    }

    fn validate_title(title: &str) -> Option<&str> {
        let title = title.trim();
        if SERIES_TITLE_REGEX.is_match(title) {
            Some(title)
        } else {
            None
        }
    }

    fn validate_name(name: &str) -> Option<&str> {
        let name = name.trim();
        if SERIES_NAME_REGEX.is_match(name) {
            Some(name)
        } else {
            None
        }
    }

    fn validate_short_name(short_name: &str) -> Option<&str> {
        let short_name = short_name.trim();
        if SERIES_SHORT_NAME_REGEX.is_match(short_name) {
            Some(short_name)
        } else {
            None
        }
    }

    fn log(&self, log: SeriesLog) {
        self.logs.borrow_mut().push(log)
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.cover_letter.as_str())?;
        f.write_str(LINE_SEP.repeat(2).as_str())?;
        let r = self
            .revisions
            .iter()
            .enumerate()
            .rev()
            .try_for_each(|(i, elt)| {
                if let Err(e) = f.write_fmt(format_args!("v{}{}", i + 2, LINE_SEP)) {
                    return ControlFlow::Break(e);
                };
                if let Err(e) =
                    f.write_fmt(format_args!("{}{}", elt.to_string().as_str(), LINE_SEP))
                {
                    return ControlFlow::Break(e);
                };
                ControlFlow::Continue(())
            });
        if let ControlFlow::Break(e) = r {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl Debug for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Name : {}{}", self.name, LINE_SEP))?;
        f.write_fmt(format_args!("Title : {}{}", self.title, LINE_SEP))?;
        f.write_fmt(format_args!("Short name : {}{}", self.short_name, LINE_SEP))?;
        f.write_fmt(format_args!("Revision : {}{}", self.current_revision(), LINE_SEP))?;
        f.write_str(LINE_SEP)?;
        Display::fmt(&self, f)?;
        f.write_fmt(format_args!("Log :{}", LINE_SEP))?;
        for l in self.logs.borrow().iter() {
            f.write_fmt(format_args!("{}{}", l.0.as_str(), LINE_SEP))?;
        }
        Ok(())
    }
}

impl SeriesRevision {
    pub fn new(content: &str) -> Self {
        Self {
            content: String::from(content),
        }
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content.trim());
    }
}

impl Display for SeriesRevision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.content.lines().try_for_each(|elt| {
            if let Err(e) = f.write_fmt(format_args!("    {}{}", elt, LINE_SEP)) {
                return ControlFlow::Break(e);
            };
            ControlFlow::Continue(())
        });
        if let ControlFlow::Break(e) = r {
            Err(e)
        } else {
            Ok(())
        }
    }
}
