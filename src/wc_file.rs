use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct WcFile {
    content: String,
}

impl WcFile {
    pub fn new(file_name: &str) -> Result<Self, String> {
        let mut file = File::open(file_name).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        Ok(Self { content })
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn lines(&self) -> usize {
        self.content.lines().count()
    }

    pub fn words_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn successful_file() {
        let wc = WcFile::new("test.txt").unwrap();
        assert_eq!(wc.words_count(), 58164);
        assert_eq!(wc.lines(), 7145);
        assert_eq!(wc.size(), 342190);
    }

    #[test]
    fn fail_file() {
        let name = "non_existent_file.txt";
        let result = WcFile::new(name).unwrap_err();
        assert_eq!(result, "No such file or directory (os error 2)".to_string());
    }
}