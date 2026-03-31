// ==========================================
// MINI BACKEND (AUTH / ORDER / PAYMENT)
// ==========================================

// ---------- DOMAIN TYPES ----------

// User roles
#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    Customer,
}

// Auth errors
#[derive(Debug)]
enum AuthError {
    InvalidCredentials,
    Unauthorized,
}

// Payment methods
#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Card { number: String },
}

// Payment errors
#[derive(Debug)]
enum PaymentError {
    _InsufficientFunds,
    InvalidCard,
}

// Order state machine
#[derive(Debug)]
enum OrderState {
    Created,
    Paid { amount: f64 },
    Failed { reason: String },
    Completed,
}

// Core user model
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    role: Role,
}

// Order model
#[derive(Debug)]
struct Order {
    id: u32,
    user_id: u32,
    amount: f64,
    state: OrderState,
}

// ---------- AUTH MODULE ----------

// authenticate user (returns Result)
fn login(username: &str, password: &str) -> Result<User, AuthError> {
    // fake DB check
    if username == "admin" && password == "1234" {
        Ok(User {
            id: 1,
            name: "jayesh".to_string(),
            role: Role::Admin,
        })
    } else if username == "user" && password == "1234" {
        Ok(User {
            id: 2,
            name: "customer".to_string(),
            role: Role::Customer,
        })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

// authorization check
fn authorize(user: &User) -> Result<(), AuthError> {
    if user.role == Role::Admin || user.role == Role::Customer {
        Ok(())
    } else {
        Err(AuthError::Unauthorized)
    }
}

// ---------- ORDER MODULE ----------

// create new order
fn create_order(user: &User, amount: f64) -> Order {
    Order {
        id: 100,
        user_id: user.id,
        amount,
        state: OrderState::Created,
    }
}

// ---------- PAYMENT MODULE ----------

// process payment
fn process_payment(order: &mut Order, method: PaymentMethod) -> Result<(), PaymentError> {
    match method {
        PaymentMethod::Cash => {
            // always succeeds
            order.state = OrderState::Paid {
                amount: order.amount,
            };
            Ok(())
        }
        PaymentMethod::Card { number } => {
            if number.len() < 4 {
                order.state = OrderState::Failed {
                    reason: "invalid card".to_string(),
                };
                Err(PaymentError::InvalidCard)
            } else {
                order.state = OrderState::Paid {
                    amount: order.amount,
                };
                Ok(())
            }
        }
    }
}

// complete order (state transition)
fn complete_order(order: &mut Order) -> Result<(), String> {
    match order.state {
        OrderState::Paid { .. } => {
            order.state = OrderState::Completed;
            Ok(())
        }
        _ => Err("order not paid".to_string()),
    }
}

// ---------- MAIN FLOW ----------

fn main() {
    println!("--- MINI BACKEND START ---");

    // STEP 1: LOGIN
    let user = match login("admin", "1234") {
        Ok(u) => {
            println!("Login success: {:?}", u);
            u
        }
        Err(e) => {
            println!("Login failed: {:?}", e);
            return;
        }
    };

    // STEP 2: AUTHORIZE
    if let Err(e) = authorize(&user) {
        println!("Unauthorized: {:?}", e);
        return;
    }

    // STEP 3: CREATE ORDER
    let mut order = create_order(&user, 500.0);
    println!("Order created: {:?}", order);

    // STEP 4: PAYMENT
    let payment_result = process_payment(
        &mut order,
        PaymentMethod::Card {
            number: "12345678".to_string(),
        },
    );

    match payment_result {
        Ok(_) => println!("Payment successful"),
        Err(e) => println!("Payment failed: {:?}", e),
    }

    println!("Order after payment: {:?}", order);

    // STEP 5: COMPLETE ORDER
    match complete_order(&mut order) {
        Ok(_) => println!("Order completed"),
        Err(e) => println!("Cannot complete order: {}", e),
    }

    println!("Final order state: {:?}", order);

    println!("--- MINI BACKEND END ---");
}
