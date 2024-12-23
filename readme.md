## impl

impl is a goup of functions
eg: 
```rust
struct User{
    name: String,
    email: String
}

impl User{
    fn new(name: &str) -> User{
        User{
            name: Name.to_string(),
            email: format!("{}@gmail.com", name)
        }
    }
}

fn main(){
    let user = User::new("srijan");// user variable is an implemenation of User
    println!("Hello {}", user.name);
    println!("Email: {}", user.email);
}
```

## Everything is immutable

you can add mutability to any variable by adding `mut` keyword.
eg:
```rust
let mut guess = String::new();
```