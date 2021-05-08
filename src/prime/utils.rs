use rocket::response::content::Json;

pub struct Prime {
    pub value: u64,
    pub position: usize,
    pub new: bool,
    pub duration: f64,
}

impl Prime {
    pub fn new(value: u64, position: usize, duration: f64, new: bool) -> Prime {
        Prime {
            value: value,
            position: position,
            duration: duration,
            new: new,
        }
    }

    pub fn to_json(&self) -> Json<String> {
        Json(format!(
            "{{ 'prime': {}, 'position': {}, 'duration': {}, 'new': {} }}",
            self.value.to_string(),
            self.position.to_string(),
            self.duration.to_string(),
            self.new.to_string()
        ))
    }
}
