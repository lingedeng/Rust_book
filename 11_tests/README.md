# The Anatomy of a Test Function
Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. 

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```
Note that we’ve added a new line inside the tests module: use super::*;.
Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.

# Checking Results
```
assert!(actual);
assert_eq!(expect, actual);
assert_ne!(expect, actual);
```
When the assertions fail, these macros print their arguments using debug formatting, 
which means the values being compared must implement the PartialEq and Debug traits.

# Custom Failure Messages
```
#[test]
fn greeting_contains_name() {
	let result = greeting("Carol");
	assert!(
		result.contains("Carol"),
		"Greeting did not contain name, value was `{}`",
		result
	);
}
```

# Checking for panics
`should_panic` attribute makes a test pass if the code inside the function panics; 
the test will fail if the code inside the function doesn’t panic.

`expect` parameter make sure that the failure message contains the provided text.
```
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
	call_func(200);
}
```

# Using Result<T, E> in Tests
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, 
which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).

# Running Tests in Parallel or Consecutively
When you run multiple tests, by default they run in parallel using threads.

```
$ cargo test -- --test-threads=1
```
We set the number of test threads to 1, telling the program not to use any parallelism.

# Showing Function Output
By default, if a test passes, Rust’s test library captures anything printed to standard output.
if we call println! in a test and the test passes, we won’t see the println! output in the terminal;

```
$ cargo test -- --show-output
```

# Running a Subset of Tests by Name
```
# Only the test with the name one_hundred ran
$ cargo test one_hundred
# Ran all tests with add in the name
$ cargo test add

#[test]
#[ignore]
// After #[test] we add the #[ignore] line to the test we want to exclude.
fn expensive_test() {
    // code that takes an hour to run
}
# run only the ignored tests
$ cargo test -- --ignored
# run all tests whether they’re ignored or not
$cargo test -- --include-ignored
```

# Unit Tests
The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with #[cfg(test)].
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.

The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option. 
In this case, the configuration option is test, which is provided by Rust for compiling and running tests.
By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test.

## Testing private functions
```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

# Integration Tests
Their purpose is to test whether many parts of your library work together correctly.
We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. 
Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

`cargo test` result contains three sections of output include the unit tests, the integration test, and the doc tests.

```
# run all the tests in a particular integration test file
$ cargo test --test integration_test
```

## Submodules in Integration Tests
example: 
	if we create tests/common.rs and place a function named `setup` in it, 
	we can add some code to setup that we want to call from multiple test functions in multiple test files.
	
	Having common appear in the test results with running 0 tests displayed for it is not what we wanted. 
	
	To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs.
	Naming the file this way tells Rust not to treat the common module as an integration test file. 
	
Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. 
Using that structure, integration tests can test the library crate with use to make the important functionality available.