use std::fs;
use std::string::String;
pub struct FileOpener{
    filename: String,
}

impl FileOpener {
    pub fn new(filename: String) -> FileOpener {
        FileOpener {
            filename
        }
    }

    pub fn get_content(&mut self) -> String {
        let contents = fs::read_to_string(&mut self.filename)
            .expect("Something went wrong reading the file");
        contents
    }
}