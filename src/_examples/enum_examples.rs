/// An enumeration for directions
#[derive(Debug, PartialEq)]
pub enum Direction {
    /// Points North
    North,
    /// Points South
    South,
    /// Points East
    East,
    /// Points West
    West,
}

/// An enumeration for traffic light states
#[derive(Debug)]
pub enum TrafficLight {
    /// Red light - Stop
    Red,
    /// Yellow light - Caution
    Yellow,
    /// Green light - Go
    Green,
}

/// An enumeration representing different types of weather
#[derive(Debug)]
pub enum Weather {
    /// Sunny weather with optional temperature
    Sunny(i32),
    /// Rainy weather with precipitation amount
    Rainy(f32),
    /// Cloudy weather
    Cloudy,
    /// Snowy weather with snowfall amount
    Snowy(f32),
}

/// An enumeration for result status
#[derive(Debug)]
pub enum Status<T, E> {
    /// Operation succeeded with value
    Success(T),
    /// Operation failed with error
    Failure(E),
    /// Operation pending
    Pending,
}
