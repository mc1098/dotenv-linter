use std::{fmt, rc::Rc};

use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    file: Rc<FileEntry>,
    line_number: usize,
    message: String,
}

impl Warning {
    pub fn new(
        line: &LineEntry,
        check_name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let check_name = check_name.into();
        let message = message.into();
        Self {
            line_number: line.number,
            file: line.file.clone(),
            check_name,
            message,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            format!("{}:{}", self.file, self.line_number).italic(),
            self.check_name.red().bold(),
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn warning_fmt_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(&line, "DuplicatedKey", "The FOO key is duplicated");

        assert_eq!(
            format!(
                "{} {}: {}",
                format!("{}:{}", ".env", "1").italic(),
                "DuplicatedKey".red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
