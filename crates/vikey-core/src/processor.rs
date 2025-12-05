//! Main processor for handling input events

use crate::{
    Action, Buffer, KeyEvent, Result, State, StateMachine, Transform, TransformResult,
};

/// Processor configuration
#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    /// Enable spell checking
    pub spell_check: bool,
    
    /// Auto-capitalize first letter
    pub auto_capitalize: bool,
    
    /// Maximum buffer size
    pub max_buffer_size: usize,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            spell_check: false,
            auto_capitalize: false,
            max_buffer_size: 32,
        }
    }
}

/// Result of processing a key event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    /// Action to perform
    pub action: Action,
    
    /// Current state after processing
    pub state: State,
}

/// Main processor for handling input
pub struct Processor {
    config: ProcessorConfig,
    state_machine: StateMachine,
    buffer: Buffer,
    transform: Transform,
}

impl Processor {
    /// Create new processor with config
    pub fn new(config: ProcessorConfig) -> Self {
        Self {
            buffer: Buffer::with_capacity(config.max_buffer_size),
            config,
            state_machine: StateMachine::new(),
            transform: Transform::new(),
        }
    }
    
    /// Process key event
    pub fn process_key(&mut self, event: KeyEvent) -> Result<ProcessResult> {
        // If modifiers are pressed (except shift), don't process
        if event.modifiers.ctrl || event.modifiers.alt || event.modifiers.meta {
            return Ok(ProcessResult {
                action: Action::DoNothing,
                state: self.state_machine.current(),
            });
        }
        
        // Get character from event
        let ch = match event.character {
            Some(c) => c,
            None => {
                return Ok(ProcessResult {
                    action: Action::DoNothing,
                    state: self.state_machine.current(),
                })
            }
        };
        
        // Transition to buffering if in initial state
        if self.state_machine.current() == State::Initial {
            self.state_machine.transition(State::Buffering)?;
        }
        
        // Add to buffer
        self.buffer.push(ch)?;
        
        // Try to transform
        let buffer_str = self.buffer.as_str();
        if let Some(result) = self.transform.apply(&buffer_str) {
            // Transformation found
            self.state_machine.transition(State::Processing)?;
            
            let action = Action::Replace {
                backspace_count: result.consumed,
                chars: result.output.chars().collect(),
            };
            
            // Update buffer
            for _ in 0..result.consumed {
                self.buffer.pop();
            }
            for ch in result.output.chars() {
                self.buffer.push(ch)?;
            }
            
            self.state_machine.transition(State::Committed)?;
            
            Ok(ProcessResult {
                action,
                state: self.state_machine.current(),
            })
        } else {
            // No transformation
            Ok(ProcessResult {
                action: Action::DoNothing,
                state: self.state_machine.current(),
            })
        }
    }
    
    /// Reset processor
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.state_machine.reset();
    }
    
    /// Get current state
    pub fn state(&self) -> State {
        self.state_machine.current()
    }
    
    /// Get buffer content
    pub fn buffer_content(&self) -> String {
        self.buffer.as_str()
    }
    
    /// Set transform engine
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KeyModifiers;

    #[test]
    fn test_processor_creation() {
        let config = ProcessorConfig::default();
        let processor = Processor::new(config);
        assert_eq!(processor.state(), State::Initial);
    }
    
    #[test]
    fn test_processor_with_modifiers() {
        let config = ProcessorConfig::default();
        let mut processor = Processor::new(config);
        
        let event = KeyEvent {
            code: 65,
            character: Some('a'),
            modifiers: KeyModifiers {
                ctrl: true,
                ..Default::default()
            },
        };
        
        let result = processor.process_key(event).unwrap();
        assert_eq!(result.action, Action::DoNothing);
    }
    
    #[test]
    fn test_processor_reset() {
        let config = ProcessorConfig::default();
        let mut processor = Processor::new(config);
        
        let event = KeyEvent {
            code: 65,
            character: Some('a'),
            modifiers: KeyModifiers::default(),
        };
        
        processor.process_key(event).unwrap();
        assert_ne!(processor.state(), State::Initial);
        
        processor.reset();
        assert_eq!(processor.state(), State::Initial);
        assert!(processor.buffer_content().is_empty());
    }
}
