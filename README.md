# Singly

Simple, Lighweight and "not" thread safe Singleton instance but it's depend on the usage, 
consult the docs for more information regarding thread safety, feel free to make thread safe wrapper

## Features

 * Set value to the instance with type.
 * Get reference value to the instance with type.
 * Get mutable reference value to the instance with type.

## Examples

```rs
fn main() {
   // Create the Singleton instance
   let mut instance = singly::Singleton::new();

   /// Set the i32 type to 12
   instance.set(12i32);

   /// Get mutable reference i32 type and set it to 14
   let a = instance.get_mut::<i32>();
   *a = 14;

   assert_eq!(instance.get::<i32>(), &14);

}
```

## Installation

```sh
cargo add singly
```

## License

This project is licensed under MIT
