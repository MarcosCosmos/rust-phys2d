use phys2d::vec2d::*;
#[test]
fn with_int() {
	let n = 4;
	let vect = Vec2D::new(n, n);
	assert!(vect.x == n && vect.y == n);
}

#[test]
fn with_float() {
	let n = 4.5;
	let vect = Vec2D::new(n, n);
	assert!(vect.x == n && vect.y == n);
}

#[test]
fn magnitude_test()
{
	let b = Vec2D::new(3, 7);
	let x = b.magnitude();
	println!("{}", x);
	assert_eq!(x, 7.615773105863909);
}

// #[test]
// fn add_vecs() {
// 	let x = phys2d::Vec2D::new(1, 1);
// 	assert_eq!(x + x, phys2d::Vec2D::new(2,2));
// }

// #[test]
// fn sub_vecs() {
// 	let x = phys2d::Vec2D::new(1, 1);
// 	assert_eq!(x - x, phys2d::Vec2D::new(0,0));
// }