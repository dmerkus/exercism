use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.data.len() < self.capacity {
            Ok(self.data.push_back(_element))
        } else {
            Err(Error::FullBuffer)
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.data.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }

        self.data.push_back(_element);
    }
}
