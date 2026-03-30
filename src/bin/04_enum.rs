// ===============================
// ENUMS IN RUST — END TO END
// ===============================

// ---------- 1. BASIC ENUM ----------
#[derive(Debug)]
enum Status {
    Online,
    Offline,
    Busy,
}

// ---------- 2. ENUM WITH DATA ----------
#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
}

// ---------- 3. STRUCT + ENUM ----------
#[derive(Debug)]
struct User {
    name: String,
    status: Status,
}

// ---------- 4. OPTION (NULL REPLACEMENT) ----------
fn find_user(flag: bool) -> Option<User> {
    if flag {
        Some(User {
            name: "jayesh".to_string(),
            status: Status::Online,
        })
    } else {
        None
    }
}

// ---------- 5. RESULT (ERROR HANDLING) ----------
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// ---------- 6. REAL MODELING (STATE MACHINE) ----------
#[derive(Debug)]
enum OrderState {
    Created,
    Paid { amount: f64 },
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

// ---------- 7. METHODS ON ENUM ----------
impl OrderState {
    // Transition logic (controlled state changes)
    fn next(self) -> Self {
        match self {
            OrderState::Created => OrderState::Paid { amount: 0.0 },
            OrderState::Paid { .. } => OrderState::Shipped {
                tracking_id: "XYZ123".to_string(),
            },
            OrderState::Shipped { .. } => self,   // terminal
            OrderState::Cancelled { .. } => self, // terminal
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self,
            OrderState::Shipped { .. } | OrderState::Cancelled { .. }
        )
    }
}

// ---------- 8. MAIN ----------
fn main() {
    // BASIC ENUM USAGE
    let status = Status::Online;

    match status {
        Status::Online => println!("User is online"),
        Status::Offline => println!("User is offline"),
        Status::Busy => println!("User is busy"),
    }

    // ENUM WITH DATA
    let msg = Message::Text("hello".to_string());

    match msg {
        Message::Text(s) => println!("Text: {}", s),
        Message::Number(n) => println!("Number: {}", n),
    }

    // STRUCT + ENUM
    let user = User {
        name: "jayesh".to_string(),
        status: Status::Offline,
    };

    // Pattern match with reference (avoid move)
    match &user.status {
        Status::Online => println!("{} is online", user.name),
        Status::Offline => println!("{} is offline", user.name),
        Status::Busy => println!("{} is busy", user.name),
    }

    // OPTION USAGE
    let maybe_user = find_user(true);

    match maybe_user {
        Some(u) => println!("Found user: {}", u.name),
        None => println!("No user found"),
    }

    // OPTION SHORTCUT
    if let Some(u) = find_user(true) {
        println!("if let user: {}", u.name);
    }

    // RESULT USAGE
    match divide(10, 2) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // RESULT + ? OPERATOR
    fn process() -> Result<(), String> {
        let v = divide(10, 2)?; // auto propagate error
        println!("Processed value: {}", v);
        Ok(())
    }

    let _ = process();

    // STATE MACHINE
    let mut order = OrderState::Created;

    println!("Initial: {:?}", order);

    order = order.next();
    println!("After payment: {:?}", order);

    order = order.next();
    println!("After shipping: {:?}", order);

    println!("Is terminal? {}", order.is_terminal());

    // MATCH GUARD (ADVANCED)
    let x = Some(15);

    match x {
        Some(v) if v > 10 => println!("Large value"),
        Some(_) => println!("Small value"),
        None => println!("No value"),
    }

    // MULTIPLE PATTERNS
    let status = Status::Online;

    match status {
        Status::Online | Status::Busy => println!("Active"),
        Status::Offline => println!("Inactive"),
    }

    // IGNORING DATA
    let msg = Message::Text("ignore me".to_string());

    match msg {
        Message::Text(_) => println!("Got text"),
        Message::Number(_) => println!("Got number"),
    }

    // DESTRUCTURING COMPLEX ENUM
    let state = OrderState::Paid { amount: 100.0 };

    match state {
        OrderState::Paid { amount } => println!("Paid: {}", amount),
        _ => {}
    }


    println!("--- ENUM LEARNING COMPLETE ---");
}
