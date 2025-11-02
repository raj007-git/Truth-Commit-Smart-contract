# TruthCommit: A Decentralized Commit-Reveal Game

---

## Project Description

TruthCommit is a decentralized, on-chain "Two Truths and a Lie" social game built on the Stellar network using Rust and Soroban smart contracts. It's a "vibe code" project for the **riseinXstellar Bootcamp**.

The game allows a user (the "Committer") to "commit" a hashed version of their secret statements and a "salt" (password). Other users ("Guessers") can then view the commitment and log their own guesses on-chain. Finally, the original committer can "reveal" their plaintext statements, and the smart contract verifies the hash on-chain to prove they didn't cheat. This serves as a fun, interactive, and multi-user demonstration of a powerful cryptographic pattern.

---

## Project Vision

The vision for this project is to create a simple, engaging, and social dApp that serves as a practical, hands-on demonstration of a core cryptographic concept: the **commit-reveal scheme**. By wrapping this powerful pattern in a familiar game ("Two Truths and a Lie"), the project aims to make complex blockchain security concepts (like those used in on-chain voting, sealed-bid auctions, and provably fair lotteries) accessible and understandable to new developers.

---

## Key Features

* **🔒 Commit-Reveal Logic:** Uses SHA-256 hashing to allow users to "commit" to an answer without revealing it, preventing the committer from changing their answer later.
* **🛡️ Client-Side Hashing:** Protects user secrets (the three statements and the secret salt) by calculating the hash on the frontend (in TypeScript) *before* sending any data to the contract.
* **🔎 On-Chain Verification:** The smart contract's `reveal` function verifies the game's outcome by re-hashing the data on-chain, ensuring the committer's proof is valid.
* **🎮 Interactive & Multi-User:** Supports multiple players with a `guess` function, allowing any user to log their guess against a specific game ID.
* **🗂️ Dynamic Game Management:** The contract can manage multiple, concurrent games using a `GameCounter` and a `Map` to store and manage each unique game's state.

---

## Future Scope

* **💰 Token Integration:** Add the ability for the game creator to lock a prize (like XLM or a custom token) into the contract, which would be automatically distributed to users who guessed correctly upon reveal.
* **🏆 Winner & Score System:** Add logic to the `reveal` function to check all stored guesses, determine the winners, and log them to a "Winners" list.
* **💾 Client-Side Storage:** Use `localStorage` in the frontend to save a user's committed statements and salt. This makes the `reveal` step seamless, as the user wouldn't have to re-type the exact data.
* **🚀 Enhanced Game Browser:** A more advanced frontend to filter active games, see which games have been revealed, and create a user profile to track "my games" and "my guesses."

## Contract_Details
 Contract_ID-CBAL7XNOHCMYFVQRUTGUY2EXRYCE3PWITOX5YNEVGBZ6NQG3BN4LAQ46
 ![alt text](<Screenshot (77).png>)