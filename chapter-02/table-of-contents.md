# Chapter 2: Language Foundations

## Overview
This chapter covers:
- Coming to grips with the Rust syntax
- Learning fundamental types and data structures
- Building command-line utilities
- Compiling programs

---

## 2.1 Creating a running program

### 2.1.1 Compiling single files with rustc
- Listing 2.1: Almost the shortest valid Rust program

### 2.1.2 Compiling Rust projects with cargo

---

## 2.2 A glance at Rust's syntax

### 2.2.1 Defining variables and calling functions
- Listing 2.2: Adding integers using variables and declaring types

---

## 2.3 Numbers

### 2.3.1 Integers and decimal (floating-point) numbers
- Listing 2.3: Numeric literals and basic operations on numbers in Rust

### 2.3.2 Integers with base 2, base 8, and base 16 notation
- Listing 2.4: Using base 2, base 8, and base 16 numeric literals
- Table 2.1: Rust types for representing scalar (single) numbers
- Table 2.2: Multiple bit patterns can represent the same number

### 2.3.3 Comparing numbers
- Table 2.3: Mathematical operators supported by Rust's numeric types
- **Callout: IMPOSSIBLE TO COMPARE DIFFERENT TYPES**
- Listing 2.5: The try_into() method converts between types
- **Callout: Floating-point hazards**

### 2.3.4 Rational, complex numbers, and other numeric types
- Listing 2.6: Calculating values with complex numbers
- Shortcut for adding a third-party dependency to a project

---

## 2.4 Flow control

### 2.4.1 For: The central pillar of iteration

### 2.4.2 Continue: Skipping the rest of the current iteration

### 2.4.3 While: Looping until a condition changes its state
- **Callout: USING WHILE TO STOP ITERATING ONCE A DURATION IS REACHED**
- Listing 2.7: Testing how fast your computer can increment a counter

### 2.4.4 Loop: The basis for Rust's looping constructs

### 2.4.5 Break: Aborting a loop
- **Callout: BREAK FROM NESTED LOOPS**

### 2.4.6 If, if else, and else: Conditional branching
- **Callout: Rust is an expression-based language**

### 2.4.7 Match: Type-aware pattern matching
- Listing 2.8: Using match to match multiple values

---

## 2.5 Defining functions
- Listing 2.9: Defining a function (extract of listing 2.2)
- Figure 2.2: Rust's function definition syntax

---

## 2.6 Using references
- Listing 2.10: Creating a reference to a large array
- Listing 2.11: Searching for an integer in an array of integers

---

## 2.7 Project: Rendering the Mandelbrot set
- Listing 2.12: Rendering the Mandelbrot set

---

## 2.8 Advanced function definitions

### 2.8.1 Explicit lifetime annotations
- Listing 2.13: A function signature with explicit lifetime annotations
- Listing 2.14: Type signature of a function with lifetime explicit annotations

### 2.8.2 Generic functions
- Listing 2.15: Type signature of a generic function
- Figure 2.3: Only a subset of types have implement addition operators
- Listing 2.16: Type signature of a generic function with trait bounds
- Listing 2.17: A generic function with a type variable and trait bounds

---

## 2.9 Creating grep-lite
- Listing 2.18: Searching for a simple pattern within lines of a string
- **Callout: Navigating Rust's rich collection of string types**
- Listing 2.19: Manually incrementing an index variable
- Listing 2.20: Automatically incrementing an index variable

---

## 2.10 Making lists of things with arrays, slices, and vectors

### 2.10.1 Arrays
- Listing 2.21: Defining arrays and iterating over their elements

### 2.10.2 Slices

### 2.10.3 Vectors
- Listing 2.22: Enabling context lines to be printed out with a Vec<Vec<T>>

---

## 2.11 Including third-party code

### 2.11.1 Adding support for regular expressions
- Listing 2.23: Matching on exact strings with the contains() method
- Listing 2.24: Searching for patterns with regular expressions

### 2.11.2 Generating the third-party crate documentation locally

### 2.11.3 Managing Rust toolchains with rustup

---

## 2.12 Supporting command-line arguments
- Listing 2.25: Adding a dependency to grep-lite/Cargo.toml
- Listing 2.26: Editing grep-lite/src/main.rs

---

## 2.13 Reading from files
- Listing 2.27: Reading a file manually line by line
- Listing 2.28: Reading a file line by line via BufReader::lines()
- Listing 2.29: Reading lines from a file

---

## 2.14 Reading from stdin
- Listing 2.30: Searching through a file or stdin

---

## Summary

Key takeaways:
- Rust has full support for primitive types, such as integers and floating-point numbers.
- Functions are strongly typed and require types to be specified for their parameters and return values.
- Rust features, such as iteration and mathematical operations, rely on traits. The for loop is a shorthand for the std::iter::IntoIterator trait, for example.
- List-like types are tailored to specific use cases. You will typically reach for Vec<T> first.
- All Rust programs have a single entry function: main().
- Every crate has a Cargo.toml file that specifies its metadata.
- The cargo tool is able to compile your code and fetch its dependencies.
- The rustup tool provides access to multiple compiler toolchains and to the language's documentation.
