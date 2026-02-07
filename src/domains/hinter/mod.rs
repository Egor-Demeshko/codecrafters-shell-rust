use std::io::{self, ErrorKind};

use rustyline::{
    Changeset, Context, Helper, completion::Completer, error::ReadlineError,
    highlight::Highlighter, hint::Hinter, line_buffer::LineBuffer, validate::Validator,
};

use crate::data_structures::trie::CommandTrie;

pub struct ShellHinter {
    pub command_trie: CommandTrie,
}

impl ShellHinter {
    pub fn new() -> Self {
        Self {
            command_trie: CommandTrie::new(),
        }
    }

    pub fn set_commands(&mut self, list: Vec<&str>) {
        for command in list.iter() {
            self.command_trie.add_command(*command);
        }
    }
}

impl Helper for ShellHinter {}

impl Validator for ShellHinter {}

impl Highlighter for ShellHinter {}

impl Completer for ShellHinter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let mut command = self.command_trie.get_first_command(line);
        if command.is_empty() {
            return Err(ReadlineError::Io(io::Error::new(
                ErrorKind::NotFound,
                "Not found",
            )));
        }
        command.push(' ');
        Ok((0, vec![command]))
    }

    /// Updates the edited `line` with the `elected` candidate.
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let end = line.pos();
        line.replace(start..end, elected, cl);
    }
}

impl Hinter for ShellHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        if pos == 0 {
            return None;
        }
        let command = self.command_trie.get_first_command(line);
        if command.is_empty() {
            return None;
        }
        if command == line {
            return None;
        }
        Some(command)
    }
}
