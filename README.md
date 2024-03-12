# Singly

Simple, Lighweight and "not" thread safe Singleton instance feel free to create a wrapper for thread safe
Currently it can :

 * Set value to the instance with type.
 * Get reference value to the instance with type.
 * Get mutable reference value to the instance with type.

Limitation :

  * Cannot be initialized at static scope since it's still not a thread safe under the hood
  (maybe there is workaround feel free to send Pull Request)

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

> [!WARNING]
> Currently not published on crates.io but you can use it by fetching directly from git

```toml
[dependencies]
singly = { git = "https://github.com/UnknownRori/singly-rs.git" }
```
