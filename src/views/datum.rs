/// A trait that defines interaction with a data point used in a bar chart.
/// This provides greater flexibility in using different data sources as one
/// can simply implement this trait and be able to use that data in a bar chart.
pub trait BarDatum {
    /// Return the category of the datum.
    fn get_category(&self) -> String;

    /// Return the value of the datum.
    fn get_value(&self) -> f32;

    /// Return the key of the datum. This is optional in a simple bar chart
    /// (just return an empty string), but is required in a stacked bar chart
    /// as the stacked entries are differentiated by the key.
    fn get_key(&self) -> String;
}

/// A trait that defines interaction with a data point used in a scatter/line plots.
pub trait PointDatum<T, U> {
    /// Return the X value.
    fn get_x(&self) -> T;

    /// Return the Y value.
    fn get_y(&self) -> U;

    /// Return the key of the datum. This is optional in a scatter/line plot with
    /// only a single type of data (just return an empty string), but is required
    /// in a chart that represents multiple categories of points.
    fn get_key(&self) -> String;
}

impl BarDatum for (f32, &str) {
    fn get_category(&self) -> String {
        String::from(self.1)
    }

    fn get_value(&self) -> f32 {
        self.0
    }

    fn get_key(&self) -> String {
        String::new()
    }
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

impl<T, U> PointDatum<T, U> for (T, U) where T: Clone, U: Clone {
    fn get_x(&self) -> T {
        self.0.clone()
    }

    fn get_y(&self) -> U {
        self.1.clone()
    }

    fn get_key(&self) -> String {
        String::new()
    }
}

impl<T, U> PointDatum<T, U> for (T, U, String) where T: Clone, U: Clone {
    fn get_x(&self) -> T {
        self.0.clone()
    }

    fn get_y(&self) -> U {
        self.1.clone()
    }

    fn get_key(&self) -> String {
        self.2.clone()
    }
}

impl<T, U> PointDatum<T, U> for (T, U, &str) where T: Clone, U: Clone {
    fn get_x(&self) -> T {
        self.0.clone()
    }

    fn get_y(&self) -> U {
        self.1.clone()
    }

    fn get_key(&self) -> String {
        String::from(self.2)
    }
}
