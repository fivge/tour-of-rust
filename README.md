## quick start

```bash
# init
cargo new xxx
# dev
cargo run
# check
cargo check
#build
cargo build
# build:prod
cargo build --release
```

```rust
fn main() {
    println!("Hello, world!");
}
```

## Types

> Scalar Types

|                        | Types                                      | Literals                      |
| ---------------------- | ------------------------------------------ | ----------------------------- |
| Signed integers        | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123i64` |
| Unsigned integers      | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10u16`           |
| Floating point numbers | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2f32`    |
| Strings                | `&str`                                     | `"foo"`, `"two\nlines"`       |
| Unicode scalar values  | `char`                                     | `'a'`, `'α'`, `'∞'`           |
| Booleans               | `bool`                                     | `true`, `false`               |

```rust
fn main() {
    let mut x: i32 = 6;
    let mut y = 101;

    print!("{x}");
    print!(" -> {x}");
}
```

> Compound Types

|        | Types                       | Literals                        |
| ------ | --------------------------- | ------------------------------- |
| Arrays | `[T; N]`                    | `[20, 30, 40]`, `[0; 3]`        |
| Tuples | `()`, `(T,)`, `(T1, T2)`, … | `()`, `('x',)`, `('x', 1.2)`, … |

```rust
fn main() {
    let mut a: [i8; 10] = [42; 10];
    let b: [i32; 4] = [1, 2, 3, 4];
    a[5] = 0;
    println!("a: {:?}", a); // [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
    println!("b: {:?}", b); // [1, 2, 3, 4]

    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0); // 7
    println!("2nd index: {}", t.1); // true
}
```

```rust
    println!("{:?}", t);
    println!("{t:?}");
    println!("{t:#?}");
```

`:?` 默认输出

`"{:?}", t` 等价于 `"{t:?}"`

`:#?` 美化输出

> 引用

```rust
    let mut x: i32 = 10;
    let mut y: i32 = 11;
    println!("x: {} {}", x, y);
    let mut ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {} {}", x, y);
    ref_x = &mut y;
    *ref_x = 21;
    println!("x: {} {}", x, y);
```

> Slices

```rust
fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let mut s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    a[2] = 31;
    s = &a[2..4];
    s = &a[2..];
    s = &a[..2];
    s = &a[..];
}
```

> str

```rust
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}

```

- &str an immutable reference to a string slice.
- String a mutable string buffer.

## Functions

```rust
fn main() {
    print_fizzbuzz_to(20);
}

/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
///
/// ### h3
fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}
```

- 函数体（或任何块）中的最后一个表达式成为返回值。 在表达式的末尾省略 `;`
- 返回类型 ` -> ()` 可省略

### if, else

```rust
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
```

### while

```rust
    while x != 1 {
        // x = x - 1;
        x -= 1;
        print!(" -> {x}");
    }
```

### for

```rust
for item in list
for i in 0..3
for i in 1..=n
```

### Rustdoc

`///` 开始

markdown 格式

### struct, impl

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
```

`self` 实例本身

### 函数重载

不支持重载

### 泛型

```rust
fn pick_one<T>(a: T, b: T) -> T {
    let id = std::process::id();
    println!("id {id}");

    if id % 2 == 0 {
        a
    } else {
        b
    }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}
```

### 类型隐式转换

```rust
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}
```

`.into()` 小类型到大类型
