# dep_crusher

Starting at a root node, traverse its dependency graph and flatten it. Nodes are a trait implementation, allowing dep_crusher to have generic, widespread use.

## Installation
There are two easy installation options.
1. Use [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) from the terminal
```bash
cargo add dep_crusher
```
2. Add the dependency to your ```Cargo.toml``` file
```toml
[dependencies]
dep_crusher = "0.1.0"
```

## Usage

1. Implement the ```dep_crusher::dep_node::Node``` trait:
```rust
#[derive(Debug)]
struct MyStruct {
    // ...
}

impl PartialEq for MyStruct {
    fn eq(&self, other: &Self) -> bool {
        // Check equality with, for example, and ID
    }
}

impl dep_crusher::dep_node::Node for MyStruct {
    type Id = u64; // Type that implements Eq + Hash;

    fn get_id(&self) -> Self::Id {
        // Get a unique identifier of MyStruct
    }

    fn get_next(&self) -> Option<Vec<Self>> {
        // Get and return the next MyStructs as an option
    }
}
```

2. Crush the dependencies!
```rust
let myStruct = MyStruct {
  // ...
}

let ordered_dependencies = myStruct.crush();
// OR
let ordered_dependencies = dep_crusher::crush(myStruct);

// Returns dep_crusher::result::Result<MyStruct>
// The Ok variant is Vec<MyStruct>
// The Error variant is dep_crusher::result::Error<MyStruct>
```

## Contributing

Pull requests are very welcome. Please feel free to make this better! For major updates, please open an issue first to discuss what you want to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
