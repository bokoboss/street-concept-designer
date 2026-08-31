use crate::error::KernelError;
use crate::policy::TolerancePolicy;

/// A finite engineering coordinate in the XY plane, expressed in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    /// X coordinate in metres.
    pub x: f64,
    /// Y coordinate in metres.
    pub y: f64,
}

impl Point2 {
    /// Construct a point. Semantic constructors validate finiteness at their boundary.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Whether both coordinates are finite.
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Euclidean distance to another point.
    pub fn distance_to(self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }

    pub(crate) fn checked(self, operation: &'static str) -> Result<Self, KernelError> {
        if self.is_finite() {
            Ok(self)
        } else {
            Err(KernelError::NonFiniteResult { operation })
        }
    }
}

/// A vector in engineering XY space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    /// X component in metres.
    pub x: f64,
    /// Y component in metres.
    pub y: f64,
}

impl Vector2 {
    /// Construct a vector.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Whether both components are finite.
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Euclidean vector length.
    pub fn length(self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Dot product.
    pub fn dot(self, other: Self) -> f64 {
        self.x.mul_add(other.x, self.y * other.y)
    }

    /// 2D scalar cross product.
    pub fn cross(self, other: Self) -> f64 {
        self.x.mul_add(other.y, -(self.y * other.x))
    }

    /// Left-hand perpendicular vector.
    pub const fn left(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Normalize with the centralized coincidence policy.
    pub fn try_normalized(self, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        policy.validate()?;
        let length = self.length();
        if !self.is_finite() || !length.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "vector normalization",
            });
        }
        if length <= policy.coordinate_coincidence_m {
            return Err(KernelError::ProjectionUndefined);
        }
        let normalized = Self::new(self.x / length, self.y / length);
        if normalized.is_finite() {
            Ok(normalized)
        } else {
            Err(KernelError::NonFiniteResult {
                operation: "normalized vector",
            })
        }
    }

    pub(crate) fn checked(self, operation: &'static str) -> Result<Self, KernelError> {
        if self.is_finite() {
            Ok(self)
        } else {
            Err(KernelError::NonFiniteResult { operation })
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::Add<Vector2> for Point2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Vector2> for Point2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Sub<Point2> for Point2 {
    type Output = Vector2;

    fn sub(self, rhs: Point2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
