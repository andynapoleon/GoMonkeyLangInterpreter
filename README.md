# 🐵 Go Monkey Interpreter 🍌

This project is still 🚧 **UNDER CONSTRUCTION** 🛠️!

Welcome to the Go Monkey Interpreter! This project is an interpreter for the Monkey Programming Language, built using Go. It is based on Thorsten Ball's "Writing An Interpreter In Go".

## 🚀 Features

- Interprets the Monkey programming language
- Supports arithmetic operations, conditionals, and more
- Written in Go for simplicity and efficiency

## 📚 About the Monkey Programming Language

Monkey is a simple, high-level programming language designed to teach the concepts of interpreters and compilers. It features:

- Variables
- Functions
- First-class and higher-order functions
- Closures
- Strings
- Arrays
- Hashes

## 🛠 Installation

To get started with the Go Monkey Interpreter, follow these steps:

1. **Clone the repository**:
    ```sh
    git clone https://github.com/andynapoleon/GoMonkeyLangInterpreter.git
    cd GoMonkeyLangInterpreter/src
    ```

2. **Build and run the project**:
    ```sh
    make run
    ```

## 📝 Usage

Once you have the interpreter running, you can enter Monkey code directly into the REPL (Read-Eval-Print Loop). Here are some examples:

```monkey
let add = fn(a, b) { a + b };
add(2, 3); // Outputs: 5

let fibonacci = fn(x) {
    if (x == 0) {
        0
    } else {
        if (x == 1) {
            1
        } else {
            fibonacci(x - 1) + fibonacci(x - 2);
        }
    }
};
fibonacci(10); // Outputs: 55
```

## 🧑‍💻 Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch: `git checkout -b my-feature-branch`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin my-feature-branch`
5. Create a pull request.

## 📞 Contact

If you have any questions, feel free to reach out:

- Email: `anhquoctran006@gmail.com`
- GitHub: [andynapoleon](https://github.com/andynapoleon)

## 📚 Acknowledgements

Special thanks to Thorsten Ball for his amazing book, ["Write An Interpreter In Go"](https://interpreterbook.com/), which inspired this project.

---

Happy coding! 🐵
