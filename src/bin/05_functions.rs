// ========================================
// RUST FUNCTIONS — END TO END
// ========================================

// ---------- 1. BASIC FUNCTION ----------
fn sum(a: i32, b: i32) -> i32 {
    a + b // implicit return (no semicolon)
}

// ---------- 2. EXPLICIT RETURN ----------
fn explicit_return(a: i32) -> i32 {
    return a * 2;
}

// ---------- 3. OWNERSHIP (MOVES VALUE) ----------
fn takes_ownership(s: String) {
    println!("Owned: {}", s);
    // s dropped here
}

// ---------- 4. BORROWING (NO OWNERSHIP TRANSFER) ----------
fn borrow_str(s: &str) -> usize {
    s.len()
}

// ---------- 5. MUTABLE BORROW ----------
fn mutate_string(s: &mut String) {
    s.push_str(" updated");
}

// ---------- 6. RETURNING OWNERSHIP ----------
fn give_back(s: String) -> String {
    s
}

// ---------- 7. MULTIPLE RETURN (TUPLE) ----------
fn get_stats() -> (i32, i32, i32) {
    (10, 20, 30)
}

// ---------- 8. GENERICS ----------
fn identity<T>(x: T) -> T {
    x
}

// ---------- 9. LIFETIMES (REFERENCE SAFETY) ----------
// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() { a } else { b }
// }

// ---------- 10. FUNCTION RETURNING RESULT ----------
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// ---------- 11. ? OPERATOR ----------
fn safe_divide() -> Result<i32, String> {
    let res = divide(10, 2)?; // propagates error
    Ok(res)
}

// ---------- 12. FUNCTION POINTER ----------
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

// ---------- 13. CLOSURES ----------
fn closures_demo() {
    let add = |a: i32, b: i32| a + b;
    println!("Closure sum: {}", add(2, 3));
}

// ---------- 14. HIGHER ORDER FUNCTION ----------
fn operate<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(10)
}

// ---------- 15. METHODS (IMPL FUNCTIONS) ----------
struct User {
    name: String,
}

impl User {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn greet(&self) {
        println!("Hello {}", self.name);
    }
}

// ---------- 16. CONST FUNCTION ----------
const fn square(x: i32) -> i32 {
    x * x
}

// ---------- 17. MAIN ----------
fn main() {
    // BASIC
    println!("Sum: {}", sum(2, 3));
    println!("Explicit: {}", explicit_return(5));

    // OWNERSHIP
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); ❌ invalid (moved)

    // BORROWING
    let s = String::from("hello");
    println!("Length: {}", borrow_str(&s));

    // MUTABLE BORROW
    let mut s = String::from("hello");
    mutate_string(&mut s);
    println!("Mutated: {}", s);

    // RETURN OWNERSHIP
    let s = give_back(s);
    println!("Returned: {}", s);

    // TUPLE RETURN
    let (a, b, c) = get_stats();
    println!("Tuple: {} {} {}", a, b, c);

    // GENERICS
    println!("Generic: {}", identity(10));
    println!("Generic str: {}", identity("hi"));

    // LIFETIME
    // let l = longest("abc", "abcdef");
    // println!("Longest: {}", l);

    // RESULT
    match divide(10, 2) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // ? OPERATOR
    println!("Safe divide: {:?}", safe_divide());

    // FUNCTION POINTER
    println!("Apply: {}", apply(add, 2, 3));

    // CLOSURES
    closures_demo();

    // HIGHER ORDER FUNCTION
    let result = operate(|x| x * 2);
    println!("Operate: {}", result);

    // METHODS
    let user = User::new("jayesh");
    user.greet();

    // CONST FN
    const VAL: i32 = square(4);
    println!("Const square: {}", VAL);

    println!("--- FUNCTION LEARNING COMPLETE ---");
}
