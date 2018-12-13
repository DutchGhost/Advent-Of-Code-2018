/// A TrackPath is a single path of a track.
#[derive(Debug)]
pub enum TrackPath {
    /// Represents `-`
    Horizontal,

    /// Represents '|'
    Vertical,

    /// Represents '\' or '/'
    Curve(CurveKind),

    /// Represents '+'. This is the point where two perpendicular paths cross
    Intersection,

    /// Not part of the track.
    Empty,
}

/// A CurveKind represents the kind of curve.
/// It is dependant of the current direction if a curve goes to the left or to the right.
#[derive(Debug)]
pub enum CurveKind {
    BottemLeftToUpRight,
    UpLeftToBottemRight,
}

impl From<char> for TrackPath {
    fn from(c: char) -> Self {
        match c {
            '-' => TrackPath::Horizontal,
            '|' => TrackPath::Vertical,
            '/' => TrackPath::Curve(CurveKind::BottemLeftToUpRight),
            '\\' => TrackPath::Curve(CurveKind::UpLeftToBottemRight),
            'v' => TrackPath::Vertical,
            '>' => TrackPath::Horizontal,
            '<' => TrackPath::Horizontal,
            '^' => TrackPath::Vertical,
            '+' => TrackPath::Intersection,
            _ => TrackPath::Empty,
        }
    }
}
