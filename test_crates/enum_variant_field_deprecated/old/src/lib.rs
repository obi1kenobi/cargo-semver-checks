pub enum Message {
    Simple,
    
    // These fields will get deprecated in new version
    Complex {
        id: u32,
        old_format: String,  // will get basic deprecation
        text: String,        // will get deprecation with message
        content: String,
    },

    // These fields had deprecation in old version
    Structured {
        valid: bool,
        
        #[deprecated]
        normal: i32,  // will lose deprecation
        
        #[deprecated = "Original message"]
        changed: i32,  // will change message
        
        #[deprecated = "Will change form"]
        still_old: i32,  // will change form but stay deprecated
    }
}

// Private enum should not trigger lint
enum Private {
    Data {
        old: String,
        new: String,
    }
}