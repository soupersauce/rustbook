use std::ops::Add;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_meters_to_millimeters() {
        let milli = Millimeters(25);
        let meter = Meters(1);

        assert_eq!(Millimeters(1025), milli + meter );
    }
    #[test]
    fn fail_add_meters_to_millimeters() {
        let milli = Millimeters(25);
        let meter = Meters(1);

        assert_eq!(Millimeters(1026), milli + meter );
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
