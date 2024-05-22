# WASM Post Filter

This project provides a WebAssembly (WASM) module written in Rust that allows for filtering blog posts on the client side. It's designed to integrate with JavaScript applications to perform efficient, case-insensitive search filtering for post titles.

## Overview

The WASM Post Filter takes a collection of posts and a search word as input. It filters the posts by the search word present in their titles and returns a maximum of the first five matches. This filter is particularly useful for implementing features like search suggestions or quick search results in web applications.

## Features

- **Case-Insensitive Search**: Converts both the post titles and the search word to lowercase for matching, ensuring the search is case-insensitive.
- **Limit Results**: Returns up to 5 matching posts to avoid overwhelming the user with too many results and to reduce processing time.
- **Clone Matching Posts**: Clones the posts that match the search criteria to a new vector to be returned.

## Usage

To use this WASM module in your JavaScript application, you need to follow these steps:

1. Compile the Rust code to WebAssembly using `wasm-pack` or a similar tool.
2. Import the compiled `.wasm` file into your JavaScript project.
3. Use the `filter_posts` function exported by this module, passing the posts and the search word as arguments.

Example:

```javascript
import { filter_posts } from './path_to_wasm_module';

const posts = [
  // ... your post objects here ...
];

const searchWord = 'rust';

const filteredPosts = filter_posts(posts, searchWord);
console.log(filteredPosts);