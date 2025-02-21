pub enum Message {
    Simple,
    
    // Fields that did not have #[deprecated] in old version
    Complex {
        id: u32,
        #[deprecated]
        old_format: String,  // Basic deprecation
        
        #[deprecated = "Use 'content' instead"]
        text: String,        // With message
        
        content: String,
    },

    // Fields that had #[deprecated] in old version.
    // Changes of attribute should NOT be reported
    Structured {
        valid: bool,
        
        // Attribute removed - should not be reported
        normal: i32,
        
        // Message changed - should not be reported 
        #[deprecated = "New message"] 
        changed: i32,
        
        // Form changed but still deprecated - should not be reported
        #[deprecated] 
        still_old: i32,
    }
}

// Private enum should not trigger lint
enum Private {
    Data {
        #[deprecated]
        old: String,
        new: String,
    }
}