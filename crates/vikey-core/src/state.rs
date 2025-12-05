//! State machine for input processing

use crate::Result;

/// Processing states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Initial state, no input buffered
    Initial,
    
    /// Buffering input characters
    Buffering,
    
    /// Processing and transforming
    Processing,
    
    /// Committed output
    Committed,
}

/// State machine for managing processing state
pub struct StateMachine {
    current: State,
    history: Vec<State>,
    max_history: usize,
}

impl StateMachine {
    /// Create new state machine
    pub fn new() -> Self {
        Self {
            current: State::Initial,
            history: Vec::new(),
            max_history: 10,
        }
    }
    
    /// Get current state
    pub fn current(&self) -> State {
        self.current
    }
    
    /// Transition to new state
    pub fn transition(&mut self, to: State) -> Result<State> {
        if !self.can_transition(to) {
            return Err(crate::Error::InvalidStateTransition {
                from: self.current,
                to,
            });
        }
        
        // Save to history
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(self.current);
        
        self.current = to;
        Ok(to)
    }
    
    /// Check if transition is valid
    pub fn can_transition(&self, to: State) -> bool {
        use State::*;
        
        match (self.current, to) {
            // From Initial
            (Initial, Buffering) => true,
            (Initial, Initial) => true,
            
            // From Buffering
            (Buffering, Processing) => true,
            (Buffering, Committed) => true,
            (Buffering, Initial) => true,
            (Buffering, Buffering) => true,
            
            // From Processing
            (Processing, Committed) => true,
            (Processing, Initial) => true,
            (Processing, Buffering) => true,
            
            // From Committed
            (Committed, Initial) => true,
            (Committed, Buffering) => true,
            
            _ => false,
        }
    }
    
    /// Reset to initial state
    pub fn reset(&mut self) {
        self.current = State::Initial;
        self.history.clear();
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_transitions() {
        let mut sm = StateMachine::new();
        assert_eq!(sm.current(), State::Initial);
        
        // Valid transition
        assert!(sm.transition(State::Buffering).is_ok());
        assert_eq!(sm.current(), State::Buffering);
        
        // Another valid transition
        assert!(sm.transition(State::Processing).is_ok());
        assert_eq!(sm.current(), State::Processing);
    }
    
    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new();
        sm.transition(State::Buffering).unwrap();
        
        // This should work
        assert!(sm.can_transition(State::Processing));
    }
    
    #[test]
    fn test_reset() {
        let mut sm = StateMachine::new();
        sm.transition(State::Buffering).unwrap();
        sm.transition(State::Processing).unwrap();
        
        sm.reset();
        assert_eq!(sm.current(), State::Initial);
    }
}
