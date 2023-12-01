use std::time::SystemTime;

pub struct Timer {
    timer : SystemTime
}

impl Timer {
    pub fn new() -> Self
    {
        Self {
            timer : SystemTime::now()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.timer.elapsed().unwrap().as_millis() as f32;
        print!("Runtime: {:5} ms ", elapsed);
    }
}
