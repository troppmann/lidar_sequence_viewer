#[derive(Default)]
pub struct ToggleRequest{
    state: bool,
    request: bool,
}
impl ToggleRequest {
    pub fn get_state(&mut self) -> bool{
        self.state
    }
    pub fn request(&mut self) {
        self.request = true;
    } 
    pub fn on_request<F>(&mut self, f: F)
    where 
        F: FnOnce(bool),
    {
        if self.request {
            self.state = !self.state;
            self.request = false;
            f(self.state);
        } 
    }
}
#[derive(Default)]
pub struct DialogRequest {
    request: bool,
    opened: bool,
}
impl DialogRequest {
    pub fn request(&mut self){
        if !self.opened {
            self.request = true;
        }
    }
    pub fn on_request<F>(&mut self, f: F)
    where
        F: FnOnce(), 
    {   
        if self.request {
            self.opened = true;
            self.request = false;
            f();
        }
    }
    pub fn is_open(&self) -> bool{
        self.opened
    } 
    pub fn closed(&mut self) {
        self.request = false;
        self.opened = false;
    }
}