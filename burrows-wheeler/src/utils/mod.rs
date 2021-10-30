use std::fs::File;
use std::io::{Read, Result as IoResult};

pub struct WindowedIterator {
    window: usize,
    read: Box<dyn Read>,
}

impl WindowedIterator {
    pub fn from_file(window: usize, file_path: &str) -> IoResult<WindowedIterator> {
        let file = File::open(file_path)?;
        let window_iter = WindowedIterator::from_read(window, Box::new(file));
        Ok(window_iter)
    }

    pub fn from_read(window: usize, read: Box<dyn Read>) -> WindowedIterator {
        WindowedIterator { window, read }
    }
}

impl Iterator for WindowedIterator {
    type Item = IoResult<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0u8; self.window];

        return match self.read.read(buffer.as_mut_slice()) {
            Ok(size) if size == 0 => None,
            Ok(size) => Some(Ok(buffer[0..size].to_vec())),
            Err(err) => Some(Err(err)),
        };
    }
}
