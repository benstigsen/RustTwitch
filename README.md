## RustTwitch

Twitch chatbot written in Rust.

---

### Features

- Threading

- Global ban protection

- Easy command management

- Color output

---

### Usage / Setup

Create a file called **credentials.txt**, add the following three lines to it:

```
oauth:some_long_token_here
botname
channel
```

*Replace the oauth token, botname and channel with your own values.*

---

Add commands to **commands.rs** in the `handle_commands()` function.

Then you can
- Respond directly with a string
- Call a function which will do / return something

---

### To-Do

- [x] Add API commands

- [x] Add easy configuration

- [x] Add delay between requests (global ban protection)

- [x] Add response loading from files
