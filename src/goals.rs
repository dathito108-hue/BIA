#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoalStatus {
    Active,
    Completed,
    Blocked,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Goal {
    pub id: u64,
    pub description: String,
    pub status: GoalStatus,
    pub created_at: u64,
    pub progress: f32,
}

#[derive(Clone, Debug)]
pub struct GoalStack {
    capacity: usize,
    goals: Vec<Goal>,
}

impl GoalStack {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            goals: Vec::new(),
        }
    }

    pub fn set(&mut self, id: u64, description: String, timestamp: u64) {
        if description.trim().is_empty() {
            return;
        }
        if self.goals.len() >= self.capacity {
            if let Some(i) = self
                .goals
                .iter()
                .position(|g| g.status != GoalStatus::Active)
            {
                self.goals.remove(i);
            } else {
                self.goals.remove(0);
            }
        }
        self.goals.push(Goal {
            id,
            description,
            status: GoalStatus::Active,
            created_at: timestamp,
            progress: 0.0,
        });
    }

    pub fn restore_active(&mut self, goal: Goal) {
        self.goals.retain(|g| g.status != GoalStatus::Active);
        self.goals.push(goal);
        if self.goals.len() > self.capacity {
            self.goals.remove(0);
        }
    }

    pub fn active(&self) -> Option<&Goal> {
        self.goals
            .iter()
            .rev()
            .find(|g| g.status == GoalStatus::Active)
    }

    pub fn update_active(&mut self, progress: f32, status: GoalStatus) {
        if let Some(goal) = self
            .goals
            .iter_mut()
            .rev()
            .find(|g| g.status == GoalStatus::Active)
        {
            goal.progress = progress.clamp(0.0, 1.0);
            goal.status = status;
        }
    }

    pub fn len(&self) -> usize {
        self.goals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.goals.is_empty()
    }
}
