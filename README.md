ZRPC is a library for creating Remote Procedure Calls (RPC) in Rust, which allows you to easily set up a server and client for data exchange. Below is a quick guide to the main aspects of using ZRPC with code examples.
#### Key Components of the library

1. **ZRpcDt Data Type**:
`ZRpcDt` is an enumeration that represents various data types used in RPC calls. It includes:

- `Int32(i32)` — an interger.
- `Float32(f32)` — a floating-point number.
- `String(String)` — a string.
- `Serialized(Vec<u8>)` — serialized data as byte vector.
- `Error(ErrorKind)` — an error that may occur during call processing.

**Code Example**:
```rust
fn add(p: &Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> {
    match (&p[1], &p[2]) {
        (ZRpcDt::Int32(a), ZRpcDt::Int32(b)) => proc_ok!(a + b),
        _ => proc_err!(InvalidParameters),
    }
}
```

2. **Serialization and Deserialization**:
Serialization and deserialization methods simplify the transfer of complex types between client and server.
**Code Example**:
```rust
User {..}.to_zdt() => ZRpcDt::Serialized([..])
```
```rust
fn user_info(p: &Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> {
    match &p[0] {
        ZRpcDt::Serialized(_) => {
            let user = p[0]
                .deserialize::<User>()
                .expect("Failed to deserialize User");

            proc_ok!(format!("Name: {}, age: {}", user.name, user.age))
        }
        _ => proc_err!(InvalidParameters),
    }
}
```

3. **ErrorKind Enumeration**:
`ErrorKind` is used to define various errors that may occur during RPC execution:

- `ProcedureNotFound` — the called procedure was not found.
- `InvalidParameters` — invalid parameters were passed.
- `InternalError` — an internal server error.

**Code Example**:
```rust
fn mul(p: &Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> {
    match (&p[0], &p[1]) {
        (ZRpcDt::Float(a), ZRpcDt::Float(b)) => proc_ok!(a * b),
        _ => proc_err!(InvalidParameters),
    }
}
```

4. **Creating a Server**:
The server is initialized using `ZRpcServer`, which listens for incoming connections.

**Code Example**:
```rust
#[tokio::main]
async fn main() {
    let mut server = ZRpcServer::new((Ipv4Addr::LOCALHOST, 3000)).await.unwrap();

    /* Adding middleware */
    server
        .add_middleware(AuthMiddleware {
            api_key: "SECRET_KEY".to_string(),
        })
        .await;
    
    /* Adding procedures */
    add_procs!(server, add, mul, user_info);
    server.start().await.unwrap()
}
```

5. **Creating a Client**:
The client is initialized using `ZRpcClient`, which establishes a connection to the server.

**Code Example**:
```rust
let mut client = ZRpcClient::new((Ipv4Addr::LOCALHOST, 3000)).await.unwrap();
```

6. **Calling Remote Procedures**:
To call remote functions, the `call` method is used. Requests are serialized and sent to the server, after which the client waits for a response.

**Code Example**:
```rust
match client.call("add", params!("SECRET_KEY", 2, 2)).await {
    Ok(ZRpcDt::Int32(res)) => println!("Sum: {}", res),
    Err(e) => eprintln!("{}", e),
    _ => {}
}
```
# Middleware
```rust
pub struct AuthMiddleware {
    api_key: String,
}

impl Middleware for AuthMiddleware {
    fn before_call(&self, req: &libzrpc::types::req::ZRpcReq) -> Result<(), MiddlewareError> {
        if let Some(ZRpcDt::String(key)) = req.1.first() {
            if key == &self.api_key {
                Ok(())
            } else {
                middleware_err!("Unauthorized")
            }
        } else {
            middleware_err!("Missing apiKey")
        }
    }
}
```
# Extensions
## Macros
To initialize a request parameters, you can also use `params!()` macro:
```rust
let _ = client.call("procedure_name", params!(..)));
```
```rust
params!("Hello, World", 1, 3.14) => vec![ZRpcDt::String("Hello, World".to_string()), ZRpcDt::Int8(1), ZRpcDt::Float(3.14)]
``` 
## Type Casting
Example:
```rust
1.to_zdt() => ZRpcDt::Int8(1)
3.14.to_zdt() => ZRpcDt::Float32(3.14)
```
