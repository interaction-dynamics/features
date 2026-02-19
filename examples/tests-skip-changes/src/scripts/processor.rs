// --feature-flag feature:feature-1, type: core, language: rust, status: stable

pub struct Processor {
    name: String,
    capacity: usize,
}

impl Processor {
    pub fn new(name: String, capacity: usize) -> Self {
        Self { name, capacity }
    }

    pub fn process(&self, data: &str) -> String {
        format!("Processing {} with {}", data, self.name)
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = Processor::new("Test".to_string(), 100);
        assert_eq!(processor.get_capacity(), 100);
    }

    #[test]
    fn test_processor_process() {
        let processor = Processor::new("Test".to_string(), 100);
        let result = processor.process("data");
        assert!(result.contains("Test"));
    }
}
