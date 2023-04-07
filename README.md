# Buss2

### What
A web app and backend for tracking bus routes and times in Agder, Norway.

### Why
The official AKT app is often slow, and loses track of favorite routes from time to time. Also, I needed an excuse to learn more Rust (specifically Axum and Diesel), and noticed bus data was available in a public API from Entur.

### How
The backend is written in Rust, using the Axum web framework, and Diesel ORM (with Postgres). The frontend is written in Vue 3, using Quasar.