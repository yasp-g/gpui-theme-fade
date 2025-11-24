#[derive(Clone, Debug, PartialEq)]
pub enum SimulationState {
    Idle,
    Sleeping { seconds_remaining: usize },
    Fading { progress: f32 },
}

impl SimulationState {
    pub fn display(&self) -> String {
        match self {
            Self::Idle => String::new(),
            Self::Sleeping { seconds_remaining } => format!("Status: Sleeping... {}s", seconds_remaining),
            Self::Fading { .. } => "Status: Fading...".to_string(),
        }
    }
}
