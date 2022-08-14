#[derive(Clone)]
pub struct ClientAccount {
  id: u16,
  available_amount: f32,
  held_amount: f32,
  is_locked: bool,
}

impl ClientAccount {
  pub fn new(id: u16) -> ClientAccount {
    ClientAccount {
      id,
      available_amount: 0.0,
      held_amount: 0.0,
      is_locked: false,
    }
  }

  pub fn get_id(&self) -> u16 {
    self.id
  }

  pub fn get_total_amount(&self) -> f32 {
    self.available_amount + self.held_amount
  }

  pub fn get_available_amount(&self) -> f32 {
    self.available_amount
  }

  pub fn adjust_available_amount(&mut self, amount: f32) {
    self.available_amount += amount;
  }

  pub fn get_held_amount(&self) -> f32 {
    self.held_amount
  }

  pub fn adjust_held_amount(&mut self, amount: f32) {
    self.held_amount += amount;
  }

  pub fn get_is_locked(&self) -> bool {
    self.is_locked
  }

  pub fn set_is_locked(&mut self, locked: bool) {
    self.is_locked = locked;
  }
}
