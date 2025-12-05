// buffer.rs - Input buffer management

/// Maximum buffer size (same as UniKey)
const BUFFER_SIZE: usize = 40;

/// Number of characters to keep when buffer is full
const KEYS_MAINTAIN: usize = 20;

/// Input buffer for managing typed characters
#[derive(Debug, Clone)]
pub struct InputBuffer {
    /// Characters in the buffer
    chars: Vec<char>,
    
    /// Lowercase flags for each character
    lowercase_flags: Vec<bool>,
    
    /// Last 'w' was converted to 'ư' (Telex specific)
    last_w_converted: bool,
    
    /// Last character was an escape character (VIQR specific)
    last_is_escape: bool,
}

impl InputBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            chars: Vec::with_capacity(BUFFER_SIZE),
            lowercase_flags: Vec::with_capacity(BUFFER_SIZE),
            last_w_converted: false,
            last_is_escape: false,
        }
    }

    /// Push a character onto the buffer
    pub fn push(&mut self, ch: char, is_lowercase: bool) {
        if self.chars.len() >= BUFFER_SIZE {
            self.throw_buffer();
        }
        self.chars.push(ch);
        self.lowercase_flags.push(is_lowercase);
    }

    /// Pop the last character from the buffer
    pub fn pop(&mut self) -> Option<(char, bool)> {
        let ch = self.chars.pop()?;
        let flag = self.lowercase_flags.pop()?;
        Some((ch, flag))
    }

    /// Get the last character without removing it
    pub fn last(&self) -> Option<&char> {
        self.chars.last()
    }

    /// Get character at specific position
    pub fn get(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    /// Set character at specific position
    pub fn set(&mut self, index: usize, ch: char) {
        if index < self.chars.len() {
            self.chars[index] = ch;
        }
    }

    /// Get the number of characters in the buffer
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// Clear the entire buffer
    pub fn clear(&mut self) {
        self.chars.clear();
        self.lowercase_flags.clear();
        self.last_w_converted = false;
        self.last_is_escape = false;
    }

    /// Get iterator over characters from a specific position
    pub fn chars_from(&self, start: usize) -> impl Iterator<Item = char> + '_ {
        self.chars[start..].iter().copied()
    }

    /// Get all characters as a string
    pub fn to_string(&self) -> String {
        self.chars.iter().collect()
    }

    /// Throw buffer - keep only last KEYS_MAINTAIN characters
    /// Called when buffer is full
    fn throw_buffer(&mut self) {
        if self.chars.len() > KEYS_MAINTAIN {
            let drain_count = self.chars.len() - KEYS_MAINTAIN;
            self.chars.drain(0..drain_count);
            self.lowercase_flags.drain(0..drain_count);
        }
    }

    /// Get/set last_w_converted flag
    pub fn last_w_converted(&self) -> bool {
        self.last_w_converted
    }

    pub fn set_last_w_converted(&mut self, value: bool) {
        self.last_w_converted = value;
    }

    /// Get/set last_is_escape flag
    pub fn last_is_escape(&self) -> bool {
        self.last_is_escape
    }

    pub fn set_last_is_escape(&mut self, value: bool) {
        self.last_is_escape = value;
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = InputBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let mut buffer = InputBuffer::new();
        buffer.push('a', true);
        buffer.push('B', false);
        
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.pop(), Some(('B', false)));
        assert_eq!(buffer.pop(), Some(('a', true)));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_last() {
        let mut buffer = InputBuffer::new();
        assert_eq!(buffer.last(), None);
        
        buffer.push('a', true);
        assert_eq!(buffer.last(), Some(&'a'));
        
        buffer.push('b', true);
        assert_eq!(buffer.last(), Some(&'b'));
    }

    #[test]
    fn test_get_set() {
        let mut buffer = InputBuffer::new();
        buffer.push('a', true);
        buffer.push('b', true);
        
        assert_eq!(buffer.get(0), Some(&'a'));
        assert_eq!(buffer.get(1), Some(&'b'));
        assert_eq!(buffer.get(2), None);
        
        buffer.set(0, 'â');
        assert_eq!(buffer.get(0), Some(&'â'));
    }

    #[test]
    fn test_clear() {
        let mut buffer = InputBuffer::new();
        buffer.push('a', true);
        buffer.push('b', true);
        buffer.set_last_w_converted(true);
        
        buffer.clear();
        
        assert_eq!(buffer.len(), 0);
        assert!(!buffer.last_w_converted());
    }

    #[test]
    fn test_to_string() {
        let mut buffer = InputBuffer::new();
        buffer.push('h', true);
        buffer.push('e', true);
        buffer.push('l', true);
        buffer.push('l', true);
        buffer.push('o', true);
        
        assert_eq!(buffer.to_string(), "hello");
    }

    #[test]
    fn test_throw_buffer() {
        let mut buffer = InputBuffer::new();
        
        // Fill buffer beyond capacity (40)
        for i in 0..50 {
            buffer.push(char::from_digit(i % 10, 10).unwrap(), true);
        }
        
        // After pushing 50 chars:
        // - First 40 chars fill buffer
        // - 41st char triggers throw_buffer, keeps last 20, then adds 41st = 21
        // - 42nd char adds to 22
        // - ...
        // - 50th char -> buffer has 30 chars
        assert_eq!(buffer.len(), 30);
    }

    #[test]
    fn test_chars_from() {
        let mut buffer = InputBuffer::new();
        buffer.push('a', true);
        buffer.push('b', true);
        buffer.push('c', true);
        
        let chars: String = buffer.chars_from(1).collect();
        assert_eq!(chars, "bc");
    }
}
