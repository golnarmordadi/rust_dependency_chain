fn main() {
    println!("Rust Dependency Chain Example with Explicit Error Handling\n");

    // Start the dependency chain
    let result = task1()
        .and_then(task2)
        .and_then(task3)
        .and_then(task4);

    match result {
        Ok(final_result) => {
            println!("\nAll tasks completed successfully!");
            println!("Final result: {}", final_result);
        }
        Err(e) => {
            println!("\nAn error occurred during task execution: {}", e);
        }
    }
}

// Task 1: The first task in the chain
fn task1() -> Result<i32, &'static str> {
    println!("Task 1: Starting...");
    let result = 10; // Simulate some work

    // Introduce an error condition
    if result == 0 {
        return Err("Task 1 failed: result was zero");
    }

    println!("Task 1: Completed with result = {}", result);
    Ok(result)
}

// Task 2: Depends on Task 1's result
fn task2(input: i32) -> Result<i32, &'static str> {
    println!("Task 2: Starting with input = {}", input);

    // Introduce a potential error based on input
    if input < 0 {
        return Err("Task 2 failed: input was negative");
    }

    let result = input + 20; // Simulate some work

    // Check for a specific failure condition
    if result > 50 {
        return Err("Task 2 failed: result exceeded 50");
    }

    println!("Task 2: Completed with result = {}", result);
    Ok(result)
}

// Task 3: Depends on Task 2's result
fn task3(input: i32) -> Result<i32, &'static str> {
    println!("Task 3: Starting with input = {}", input);

    // Introduce a potential error based on input
    if input % 2 != 0 {
        return Err("Task 3 failed: input was not even");
    }

    let result = input * 2; // Simulate some work

    println!("Task 3: Completed with result = {}", result);
    Ok(result)
}

// Task 4: Depends on Task 3's result
fn task4(input: i32) -> Result<i32, &'static str> {
    println!("Task 4: Starting with input = {}", input);

    // Introduce a potential error based on input
    if input == 0 {
        return Err("Task 4 failed: input was zero");
    }

    let result = input / 2; // Simulate some work

    println!("Task 4: Completed with result = {}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let result = task1();
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_task2() {
        let result = task2(10);
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_task2_failure() {
        let result = task2(40);
        assert_eq!(result, Err("Task 2 failed: result exceeded 50"));
    }

    #[test]
    fn test_task3() {
        let result = task3(30);
        assert_eq!(result, Ok(60));
    }

    #[test]
    fn test_task3_failure() {
        let result = task3(31);
        assert_eq!(result, Err("Task 3 failed: input was not even"));
    }

    #[test]
    fn test_task4() {
        let result = task4(60);
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_task4_failure() {
        let result = task4(0);
        assert_eq!(result, Err("Task 4 failed: input was zero"));
    }

    #[test]
    fn test_full_chain_success() {
        let result = task1()
            .and_then(task2)
            .and_then(task3)
            .and_then(task4);

        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_full_chain_failure_at_task2() {
        let result = task1()
            .and_then(|res| task2(40))
            .and_then(task3)
            .and_then(task4);

        assert_eq!(result, Err("Task 2 failed: result exceeded 50"));
    }
}
