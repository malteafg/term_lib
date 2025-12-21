pub trait Command {
    fn get_char(&self) -> char;
    fn get_info(&self) -> &str;

    fn display_as_cmd(&self) -> String {
        format!("{}: {}", self.get_char(), self.get_info())
    }
}

pub struct CmdList<C: Command + Ord> {
    cmds: Vec<C>,
}

impl<C: Command + Ord> CmdList<C> {
    pub fn new(mut cmds: Vec<C>) -> Self {
        cmds.sort();
        CmdList { cmds }
    }

    pub fn get(&self, char: char) -> Option<&C> {
        self.cmds.iter().find(|&cmd| cmd.get_char() == char).map(|v| v as _)
    }
}

impl<T: Command> Command for &T {
    fn get_char(&self) -> char {
        (*self).get_char()
    }

    fn get_info(&self) -> &str {
        (*self).get_info()
    }
}

impl<C: Command + Ord> core::ops::Deref for CmdList<C> {
    type Target = Vec<C>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.cmds
    }
}

impl<C: Command + Ord> core::ops::DerefMut for CmdList<C> {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.cmds
    }
}
