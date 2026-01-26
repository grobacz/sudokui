const HISTORY_CAPACITY: usize = 50;

#[derive(Debug, Clone)]
pub enum GameAction {
    #[allow(dead_code)]
    SetCell {
        #[allow(dead_code)]
        row: usize,
        #[allow(dead_code)]
        col: usize,
        #[allow(dead_code)]
        old_value: Option<u8>,
        #[allow(dead_code)]
        new_value: Option<u8>,
    },
    #[allow(dead_code)]
    ToggleCandidate {
        #[allow(dead_code)]
        row: usize,
        #[allow(dead_code)]
        col: usize,
        #[allow(dead_code)]
        digit: u8,
    },
    #[allow(dead_code)]
    ClearCandidates {
        #[allow(dead_code)]
        row: usize,
        #[allow(dead_code)]
        col: usize,
        #[allow(dead_code)]
        old_mask: u16,
    },
    #[allow(dead_code)]
    NewGame,
}

#[derive(Debug, Clone)]
pub struct ActionHistory {
    past: Vec<GameAction>,
    future: Vec<GameAction>,
}

impl ActionHistory {
    pub fn new() -> Self {
        Self {
            past: Vec::with_capacity(HISTORY_CAPACITY),
            future: Vec::with_capacity(HISTORY_CAPACITY),
        }
    }

    pub fn push(&mut self, action: GameAction) {
        if self.past.len() >= HISTORY_CAPACITY {
            self.past.remove(0);
        }
        self.past.push(action);
        self.future.clear();
    }

    pub fn undo(&mut self) -> Option<GameAction> {
        let action = self.past.pop();
        if let Some(ref a) = action {
            self.future.push(a.clone());
        }
        action
    }

    pub fn redo(&mut self) -> Option<GameAction> {
        let action = self.future.pop();
        if let Some(ref a) = action {
            self.past.push(a.clone());
        }
        action
    }
}

impl Default for ActionHistory {
    fn default() -> Self {
        Self::new()
    }
}
