pub trait BarDatum {
    fn get_category(&self) -> String;
    fn get_value(&self) -> f32;
    fn get_key(&self) -> String;
}

impl BarDatum for (String, f32, String) {
    fn get_category(&self) -> String {
        String::from(&self.0)
    }

    fn get_value(&self) -> f32 {
        self.1
    }

    fn get_key(&self) -> String {
        String::from(&self.2)
    }
}

impl BarDatum for (&str, f32, &str) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1
    }

    fn get_key(&self) -> String {
        String::from(self.2)
    }
}

impl BarDatum for (String, f32) {
    fn get_category(&self) -> String {
        String::from(&self.0)
    }

    fn get_value(&self) -> f32 {
        self.1
    }

    fn get_key(&self) -> String {
        String::new()
    }
}

impl BarDatum for (&str, f32, String) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1
    }

    fn get_key(&self) -> String {
        String::from(&self.2)
    }
}

impl BarDatum for (&str, f32) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1
    }

    fn get_key(&self) -> String {
        String::new()
    }
}

impl BarDatum for (&str, i32, String) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1 as f32
    }

    fn get_key(&self) -> String {
        String::from(&self.2)
    }
}

impl BarDatum for (&str, i32) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1 as f32
    }

    fn get_key(&self) -> String {
        String::new()
    }
}

impl BarDatum for (&str, i32, &str) {
    fn get_category(&self) -> String {
        String::from(self.0)
    }

    fn get_value(&self) -> f32 {
        self.1 as f32
    }

    fn get_key(&self) -> String {
        String::from(self.2)
    }
}
