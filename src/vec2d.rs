use std;
use std::ops::*;
use std::fmt::Debug;
/// a fairly straight-forward 2D vector type (in the mathematical sense), generally supporting:
/// PartialEq, partialordering, addition, subtraction, scalar multiplication, scalar division, and dot-product operations
/// Note: a default implementation is provided for these implementations but these are provide when the support of the relevant traits/operations is present in the scalar types used
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default {
	pub x: T,
	pub y: T
}


impl<T> Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default,  {
	/// Creates a new `Vec2D` by assigning the parameters to the x and y members respectively.
	pub fn new(x: T, y: T) -> Vec2D<T> {
		Vec2D{x: x, y: y}
	}

	/// Dot product, AKA scalar vector product, this takes two vectors and returns their contained scalar type be summing the products of each dimension.
	/// Default implementation relies on the relevant operations in the underlying type.
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, u,v) = (3,3, 3,3);
	/// 	let (a, b) = (Vec2D::new(x,y), Vec2D::new(u,v));
	/// 	assert_eq!(Vec2D::dot_product(a, b), 18);
	/// ```
	pub fn dot_product(a: Vec2D<T>, b: Vec2D<T>) -> T where T: Add<Output=T>+Mul<Output=T> {
		a.x*b.x + a.y*b.y
	}

	/// determines the magnitude of the vector's polar form in terms it's scalar type.
	/// # Examples
	/// ```
	/// 	use phys2d::vec2d::Vec2D;
	/// 	let b = Vec2D::new(3, 7);
	/// 	assert_eq!(b.magnitude(), 7.615773105863909);
	/// ```
	pub fn magnitude (&self) -> f64 where T: Add<Output=T>+Mul<Output=T>, f64: std::convert::From<T> {
		f64::from(self.x*self.x + self.y*self.y).sqrt()
	}

	/// gets the angle of the vector's polar form
	/// TODO: CREATE TESTS FOR THIS
	/// #Examples
	/// ```
	/// 	use phys2d::vec2d::Vec2D;
	/// 	assert_eq!(Vec2D::new(0, 1).angle(), std::f64::consts::PI/2f64);
	/// ```
	pub fn angle (&self) -> f64 where T: Add<Output=T>+Mul<Output=T>, f64: std::convert::From<T> {
		return f64::from(self.y).atan2(f64::from(self.x));
	}

	/// Creates a unit equivilant of the vector (same direction, magnitude 1)
	/// #Examples
	/// ```
	///		use std;
	/// 	use phys2d::vec2d::Vec2D;
	/// 	let a = Vec2D::new(5,5);
	/// 	let b = a.to_unit();
	/// 	assert_eq!(a.angle(), b.angle());
	/// 	assert!(b.magnitude()-1f64 < std::f64::EPSILON);
	/// 	assert!(a.magnitude() != 1f64);
	/// ```
	pub fn to_unit(self) -> Vec2D<f64> where T: Add<Output=T>+Mul<Output=T>, f64: std::convert::From<T> {
		let mag = self.magnitude();
		Vec2D::new(f64::from(self.x)/mag, f64::from(self.y)/mag)
	}

	/// Projects a onto b
	/// #Examples
	/// ```
	///		use phys2d::vec2d::Vec2D;
	/// 	let a = Vec2D::new(5,5);
	///		assert_eq!(a.project_onto(Vec2D::new(0,1)), Vec2D::new(0f64,5f64));
	/// ```
	pub fn project_onto(self, b: Vec2D<T>) -> Vec2D<f64> where T: Add<Output=T>+Mul<Output=T>, Vec2D<f64>: std::convert::From<Vec2D<T>>, f64: std::convert::From<T>
	{
		let unit_b = b.to_unit();
		unit_b*(Vec2D::dot_product(<Vec2D<f64>>::from(self), unit_b))
	}

	///gets an all-positive version of the vector
	pub fn abs(self) -> Vec2D<T>
	{
		unimplemented!();
	}
}

impl std::convert::From<Vec2D<i32>> for Vec2D<f64> {
	fn from(src: Vec2D<i32>) -> Vec2D<f64> {
		Vec2D::new((src.x as f64), (src.y as f64))
	}
}

impl<T> Add<Vec2D<T>> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + Add<Output=T>, {
	type Output = Vec2D<T>;
	/// Adds two `Vec2D` together (by summing their members)
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, u,v) = (3,3, 3,3);
	/// 	let (a, b) = (Vec2D::new(x,y), Vec2D::new(u,v));
	/// 	assert_eq!(a+b, Vec2D::new(6, 6));
	/// ```
	fn add(self, rhs: Vec2D<T>) -> Vec2D<T> {
		Vec2D::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl<T> AddAssign<Vec2D<T>> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + AddAssign {
	/// Adds two `Vec2D` togther and stores the result into the first
	/// # Examples
	/// ```
	/// 	use phys2d::vec2d::Vec2D;
	/// 	let (x,y, u,v) = (3,3, 2,2);
	/// 	let (mut a, b) = (Vec2D::new(x,y), Vec2D::new(u,v));
	/// 	a += b;
	/// 	assert_eq!(a, Vec2D::new(5,5));
	/// ```
	fn add_assign(&mut self, rhs: Vec2D<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Sub<Vec2D<T>> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + Sub<Output=T> {
	/// Subtracts one `Vec2D` from another (by subtracting their members)
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, u,v) = (3,3, 2,2);
	/// 	let (a, b) = (Vec2D::new(x,y), Vec2D::new(u,v));
	/// 	assert_eq!(a-b, Vec2D::new(1, 1));
	/// ```
	type Output = Vec2D<T>;
	fn sub(self, rhs: Vec2D<T>) -> Vec2D<T> {
		Vec2D::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl<T> SubAssign<Vec2D<T>> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + SubAssign {
	/// Subtracts one `Vec2D` from another and stores the result in the first
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, u,v) = (3,3, 2,2);
	/// 	let (mut a, mut b) = (Vec2D::new(x,y), Vec2D::new(u,v));
	///		a -= b;
	/// 	assert_eq!(a, Vec2D::new(1, 1));
	/// ```
	fn sub_assign(&mut self, rhs: Vec2D<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}
impl<T> Mul<T> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + Mul<Output=T> {
	type Output = Vec2D<T>;

	/// Multiplies a `Vec2D` by it's scalar `T` (by multiplying it's members by the scalar)
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, s) = (3,3, 2);
	/// 	let a = Vec2D::new(x,y);
	/// 	assert_eq!(a*s, Vec2D::new(6, 6));
	/// ```
	fn mul(self, rhs: T) -> Vec2D<T> {
		Vec2D::new(self.x * rhs, self.y * rhs)
	}
} 

impl<T> MulAssign<T> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + MulAssign {

	/// Multiplies a `Vec2D` by it's scalar `T` and stores the result into the `Vec2D`
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, s) = (3,3, 2);
	/// 	let mut a = Vec2D::new(x,y);
	///     a *= s;
	/// 	assert_eq!(a, Vec2D::new(6, 6));
	/// ```
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> Div<T> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + Div<Output=T> {
	type Output = Vec2D<T>;

	/// Divides a `Vec2D` by it's scalar `T` (by dividing it's members by the scalar)
	/// Note: truncates by default if the underlying types truncate
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, s) = (7,7, 2);
	/// 	let a = Vec2D::new(x,y);
	/// 	assert_eq!(a/s, Vec2D::new(3, 3));
	///
	///		let b = Vec2D::new(x as f64, y as f64);
	/// 	assert_eq!(b/(s as f64), Vec2D::new(3.5, 3.5));
	/// ```
	fn div(self, rhs: T) -> Vec2D<T> {
		Vec2D::new(self.x / rhs, self.y / rhs)
	}
}


impl<T> DivAssign<T> for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + DivAssign {

	/// Divides a `Vec2D` by it's scalar `T` and stores the result into the `Vec2D`
	/// Note: truncates by default if the underlying types truncate
	/// # Examples
	/// ```
	///     use phys2d::vec2d::Vec2D;
	/// 	let (x,y, s) = (7,7, 2);
	/// 	let mut a = Vec2D::new(x,y);
	///		a /= s;
	/// 	assert_eq!(a, Vec2D::new(3, 3));
	///
	///		let mut b = Vec2D::new(x as f64, y as f64);
	///		b /= (s as f64);
	/// 	assert_eq!(b, Vec2D::new(3.5, 3.5));
	/// ```
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl<T> Neg for Vec2D<T> where T: Copy+Debug+PartialEq+PartialOrd+Default + Neg<Output=T>
{
	type Output = Self;
	///TODO: CREATE UNIT TEST FOR THIS
	fn neg(self) -> Vec2D<T> {
		Vec2D::new(-self.x, -self.y)
	}
}

