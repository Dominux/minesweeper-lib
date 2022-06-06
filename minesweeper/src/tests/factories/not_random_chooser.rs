use crate::random_chooser::RandomChooser;

#[derive(Clone, PartialEq)]
pub(crate) struct NotRandomChooser {
    choosen_result: Vec<usize>,
}

impl NotRandomChooser {
    pub fn new(choosen_result: Vec<usize>) -> Self {
        Self { choosen_result }
    }
}

impl RandomChooser for NotRandomChooser {
    fn choose_multiple(&self, _vec: Vec<usize>, _amount: usize) -> Vec<usize> {
        self.choosen_result.clone()
    }
}
