pub struct Stack<Element>{
    entries: [Element;0xFFFF],
    current_index: usize,
}

impl<Element> Stack<Element> {
    pub fn new(entries: [Element; 0xFFFF]) -> Stack<Element> {
        Stack{
            current_index:0,
            entries
        }
    }

    pub fn pop(&mut self)-> &Element {
        let element = &self.entries[self.current_index];
        self.current_index -=1;
        element
    }

    pub fn push(&mut self, element: Element) {
        self.current_index += 1;
        self.entries[self.current_index] = element;
    }

    pub fn get_index(&mut self) -> &usize {
        &mut self.current_index
    }
}