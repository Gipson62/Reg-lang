# WARNING

This are just some ideas on how to implement the Libraries in the language. This is maybe a """new pattern""" derived from the ECS, we can call it Library Component Function (LCF for short) because I don't find any pattern that really fit my need.

# BasicLib Trait

```rs
trait LibraryComponent {
    // Loads the library's functions into the HashMap
    fn load(&mut self);
    // Unloads the library's functions from the HashMap
    fn unload(&mut self);
    // Clears the HashMap of all functions
    fn clear(&mut self);
    // Returns a reference to the function with the given name, if it exists
    fn get_function(&self, name: &str) -> Option<&Fn(...)>;
}
```
To use the Math library, you would first create an instance of the MathLibrary struct and call the load method to load the available functions. Then, you can call the desired function using the functions HashMap. For example:

```rs
// Create an instance of the MathLibrary struct
let mut math = MathLibrary { functions: HashMap::new() };

// Load the available functions
math.load();

// Call the add function
let result = (math.functions.get("add").unwrap())(2.0, 3.0);

// Print the result
println!("The result is: {}", result); // Output: The result is: 5
```

This implementation allows you to easily add and remove functions from the Math library, as well as call them transparently from your code.

```rs
// Represents a function in the library.
type Function = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;
// Represents the value of a function argument or return value.
type Value = String;

// Implements the LibraryComponent trait for a library component.
struct LibraryComponentImpl {
    // The name of the library.
    name: String,
    // The functions in the library.
    functions: HashMap<String, Function>,
}

impl LibraryComponent for LibraryComponentImpl {
    fn load(&mut self) -> Result<(), String> {
        // Load the library into memory.
        Ok(())
    }

    fn unload(&mut self) -> Result<(), String> {
        // Unload the library from memory.
        Ok(())
    }

    fn get_function(&self, name: &str) -> Result<&Function, String> {
        // Retrieve the function from the library by name.
        Ok(self.functions.get(name).ok_or_else(|| format!("Function not found: {}", name))?)
    }
}
```

# Generics Type :

```rs
use std::collections::HashMap;

// Define the struct for the Math library
struct MathLibrary<T> {
    // Define a HashMap to hold the list of available functions
    functions: HashMap<String, Box<dyn Fn(T, T) -> T>>,
}

// Implement the load method for the Math library
impl<T> MathLibrary<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + std::ops::Pow<Output = T>,
{
    fn load(&mut self) {
        // Add the available functions to the HashMap
        self.functions.insert(String::from("add"), Box::new(add));
        self.functions.insert(String::from("sub"), Box::new(sub));
        self.functions.insert(String::from("mul"), Box::new(mul));
        self.functions.insert(String::from("div"), Box::new(div));
        self.functions.insert(String::from("pow"), Box::new(pow));
    }
}

// Define the add function
fn add<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Add<Output = T>,
{
    a + b
}

// Define the sub function
fn sub<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Sub<Output = T>,
{
    a - b
}

// Define the mul function
fn mul<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Mul<Output = T>,
{
    a * b
}

// Define the div function
fn div<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Div<Output = T>,
{
    a / b
}

// Define the pow function
fn pow<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Pow<Output = T>,
{
    a.powf(b)
}
```

# Don't know

Here is a base struct for a LibraryComponent:

```rs
struct LibraryComponent {
    name: String,
    version: String,
    description: String,
    functions: HashMap<String, Function>
}
```

To extend this struct for a specific library, such as the Math library, you would create a new struct that has the same fields as the LibraryComponent struct, plus any additional fields that are specific to the Math library. For example:

```rs
struct MathComponent {
    name: String,
    version: String,
    description: String,
    functions: HashMap<String, Function>,
    // additional fields specific to the Math library
}
```

To use the LibraryComponent trait with the MathComponent struct, you would implement the trait for the MathComponent struct, like this:

```rs
impl LibraryComponent for MathComponent {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
}
```

This implementation of the LibraryComponent trait allows you to use the methods defined in the trait with instances of the MathComponent struct. For example, you could call the name method on a MathComponent instance to get the name of the library, like this:

```rs
let math = MathComponent {
    // initialize fields
};

let name = math.name();
```

You can also use the LibraryComponent trait to create generic functions or data types that can work with any library that implements the trait. This allows for greater flexibility and reuse of code. For example, you could create a generic function that takes a LibraryComponent as an argument and prints the name and description of the library.

# Ideas

## Typing and Constant additions

To add some types we could implement an other field to the Library struct, something like `types`. A simple Hashmap who hold the name of the type as a string and the struct of the type.

```rs
struct Library<T> {
    // Name of the lib
    name: String,
    // Functions of the library
    functions: HashMap<String, Box<dyn fn(T, T) -> T>>,
    // Types of the library
    types: HashMap<String, Box<dyn TypeInfo>>
    // Constant of the library
    constants: HashMap<String, Box<dyn ConstantExample>>
}
// WARNING CHECK THE T TYPE
struct TypeInfo<T> {
    // Name of the type
    name: String,
    // Size of the type in bytes
    size: usize,
    // The function to create an instance of the type you want like in rust for the String::new()
    // For example you can create a Point type
    create: fn() -> Box<T>, //Check if T can be used here ....
    // A HashMap of the available functions for the type
    functions: HashMap<String, Box<dyn Fn(T, T) -> T>>
}
impl<T> TypeInfo<T> {
    pub fn new(name: &str, size: usize, create: fn() -> Box<T>, functions: HashMap<String, Box<dyn Fn(T, T) -> T>>) -> TypeInfo<T> {

    }
}

struct ConstantExample<T> {
    value: T,
    // ? something else ?
}
```

## Sublibraries

Don't forget to add the features of sublibraries, like just load a part of the library (with hierarchy)