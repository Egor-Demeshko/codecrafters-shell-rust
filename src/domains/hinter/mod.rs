use std::{
    cell::{Cell, RefCell},
    time::{Duration, Instant},
};

use rustyline::{
    Changeset, Context, Helper, completion::Completer, error::ReadlineError,
    highlight::Highlighter, hint::Hinter, line_buffer::LineBuffer, validate::Validator,
};

use crate::data_structures::trie::CommandTrie;

pub struct ShellHinter {
    pub command_trie: CommandTrie,
    pub tab_quantity: Cell<u8>,
    pub last_tab_time: RefCell<Instant>,
}

impl ShellHinter {
    pub fn new() -> Self {
        Self {
            command_trie: CommandTrie::new(),
            tab_quantity: Cell::new(0),
            last_tab_time: RefCell::new(Instant::now()),
        }
    }

    pub fn set_commands(&mut self, list: Vec<String>) {
        for command in list.iter() {
            self.command_trie.add_command(command);
        }
    }

    pub fn increase_tab(&self) {
        let current = self.tab_quantity.get();
        let mut time = self.last_tab_time.borrow_mut();

        if current == 1 && time.elapsed() > Duration::from_secs(1) {
            self.reset_tab();
        } else {
            self.tab_quantity.set(current + 1);
        }
        *time = Instant::now();
    }

    pub fn reset_tab(&self) {
        self.tab_quantity.set(0);
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
        _: usize,
        _: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        self.increase_tab();
        let commands: Vec<String> = self.command_trie.get_all_for_prefix(line);
        if commands.is_empty() {
            return Ok((0, vec![format!("{}{}", line, '\x07')]));
        } else if commands.len() == 1 {
            return Ok((0, vec![format!("{} ", commands.get(0).unwrap())]));
        }
        Ok((0, commands))
    }

    /// Updates the edited `line` with the `elected` candidate.
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let end = line.pos();
        line.replace(start..end, elected, cl);
    }
}

impl Hinter for ShellHinter {
    type Hint = String;

    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<Self::Hint> {
        return None;
        // if pos == 0 {
        //     return None;
        // }
        // let command = self.command_trie.get_first_command(line);
        // if command.is_empty() {
        //     return None;
        // }
        // if command == line {
        //     return None;
        // }
        // Some(command)
    }
}
