#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect_a = Rectangle {
    width: 30,
    height: 50,
  };

  let rect_b = Rectangle {
    width: 10,
    height: 20,
  };

  let rect_c = Rectangle::square(100);

  println!(
    "The area: {} square pixels",
    rect_a.area(),
  );

  println!("Can Rect hold Box? {}", rect_a.can_hold(&rect_b));

  dbg!(&rect_a, &rect_b, &rect_c);
}
