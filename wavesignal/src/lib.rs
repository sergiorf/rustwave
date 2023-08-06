// Define a struct to represent a discrete-time signal
pub struct Signal {
    samples: Vec<f64>,
}

impl Signal {
    // Constructor to create a new Signal from a vector of samples
    pub fn new(samples: Vec<f64>) -> Self {
        Signal { samples }
    }

    // Accessor method to get the samples of the signal
    pub fn samples(&self) -> &[f64] {
        &self.samples
    }

    // Method to get the length of the signal
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    // Method to get the value of a sample at a given index
    pub fn get_sample(&self, index: usize) -> Option<f64> {
        self.samples.get(index).copied()
    }

    // Method to set the value of a sample at a given index
    pub fn set_sample(&mut self, index: usize, value: f64) -> Option<()> {
        if let Some(sample) = self.samples.get_mut(index) {
            *sample = value;
            Some(())
        } else {
            None
        }
    }

    // Method to perform some basic signal processing operations
    // Add more methods as you build out your library
    // For example, methods for filtering, transforming, etc.
    // can be added here.
}
