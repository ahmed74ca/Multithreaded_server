# **Embedded Recruitment Task**

## **Overview**

Welcome to the **Embedded Recruitment Task**! This task is designed to assess your ability to analyze, debug, and enhance a server application written in Rust. The objective is to transition the server from a buggy, single-threaded implementation to a robust, multithreaded architecture capable of handling concurrent clients efficiently.

## **Task Description**

### **What You’ll Be Working On**
You’ll be provided with:
- A buggy, single-threaded Rust server implementation.
- A client test suite to evaluate the server’s behavior.

Your task:
1. Debug and fix the existing server code.
2. Transition the server from single-threaded to multithreaded.
3. Enhance the server to handle multiple clients concurrently while maintaining data consistency.
4. Use and extend the provided test suite to ensure the server meets all requirements.

### **Objectives**
1. **Analyze the Existing Server Code:**
   - Identify and fix intentional bugs in the provided implementation.
   - Understand the limitations of the single-threaded architecture.

2. **Transition to Multithreading:**
   - Modify the server to handle multiple clients using Rust’s multithreading libs.
   - Implement proper synchronization mechanisms to ensure thread safety.

3. **Enhance Server Architecture:**
   - Resolve any architectural flaws related to single-threaded assumptions.
   - Optimize the server for scalability and concurrent request handling.

4. **Testing and Validation:**
   - Run the provided test suite to verify functionality.
   - Augment the test suite with additional test cases to cover edge cases and concurrency scenarios.

5. **Demonstrate Code Quality:**
   - Write clean, maintainable, and well-documented code.
   - Provide comments and documentation to explain your changes.

# Submission Instructions
Once you have completed the task to the best of your ability:

1. Prepare your solution, including:
   - The updated server implementation.
   - The test results and any additional test cases you added.
   - Documentation (see the Deliverables section for details).
2. Send your solution as a response to the recruitment email you received.
   - Please ensure all required files are attached, or provide a link to a hosted repository (e.g., GitHub, GitLab) if your solution is hosted online.

We understand the task may be challenging, and all solutions will be evaluated based on effort, approach, and quality. Even partial solutions that demonstrate thoughtful design and debugging skills are welcome.

## **Requirements**

### **Functional Requirements**
- The server shall:
  - Handle multiple clients concurrently.
  - Maintain data consistency and avoid race conditions, deadlocks, and starvation.
  - Log meaningful error messages and warnings.

### **Technical Requirements**
- Use Rust’s standard multithreading libraries or async runtime for concurrency.
- Implement synchronization mechanisms to ensure thread safety.
- Ensure the server adheres to performance requirements without unnecessary delays.

## **Getting Started**

### **Prerequisites**
- Basic Rust knowledge [Rust book](https://doc.rust-lang.org/book/title-page.html)
- Install Rust (latest stable version recommended). [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Familiarity with Rust multithreading and asynchronous programming concepts. [Rust Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- Install protoc to compile a protobuf message [Protocol buffers](https://protobuf.dev/overview/)

### **Repository Structure**
```plaintext
.
|── proto/
│   └── messages.proto        # IDL with messages server handle
├── src/
│   ├── main.rs               # Server implementation (single-threaded and buggy)
│   └── lib.rs                # Core server logic
├── tests/
│   └── client_test.rs        # Client test suite
├── .gitignore
├── build.rs                  # Build script for compiling the Proto file
├── Cargo.toml                # Rust dependencies and configuration
├── README.md                 # Task instructions
└── SOLUTION.md               # Place for your findings and analysis
```

## Running Tests

To run the provided test suite:

```bash
cargo test
```

## Deliverables

1. Updated Server Implementation
   - Fully functional server that adheres to the multithreading requirements.
2. Test Suite Results
   - Evidence (e.g., logs) that your server passes all tests.
   - Any additional test cases you added to the test suite.
3. Documentation:
   - Inline comments in the code to explain significant changes.
   - Bug Analysis and Fix Report
4. A brief document outlining:
   - The identified bugs in the initial implementation.
   - How architectural flaws were addressed.

## Evaluation Criteria

Your submission will be evaluated based on the following criteria:
- **Correctness**: Does the updated server meet the functional and technical requirements?
- **Bug Fixes**: How effectively were the provided bugs identified and resolved?
- **Design Improvement**: Was the transition to multithreading executed appropriately? Were architectural flaws addressed?
- **Testing**: Does the server pass all provided and additional tests? Are the new tests meaningful and comprehensive?
- **Code Quality**: Is the code clean, maintainable, and well-documented?

## Good Luck!

We’re excited to see how you approach this task. If you have any questions or run into issues, don’t hesitate to reach out.
