![alt text](Logo.png)
# orgzr-3000

A headless, modular assistant written in Rust for organizing daily life.

`orgzr-3000` is built on the principle of separating business logic from the user interface. The core engine is a UI-agnostic library, designed to be controlled by various clients such as a Command-Line Interface (CLI) or a Terminal User Interface (TUI).

## Use Cases

`orgzr-3000` is intended to help with a variety of personal organization tasks, including but not limited to:
-   Managing daily tasks and to-do lists.
-   Planning weekly meals and generating shopping lists.
-   Tracking personal budgets and expenses.
-   Scripting and automating personal routines through the CLI.

## Features

-   **Headless Core:** All application logic is contained within the `orgzr-core` library, with no knowledge of how it is presented.
-   **Modular Architecture:** Functionality is divided into independent modules (`plugs`) that are known at compile time, ensuring a robust and maintainable codebase.
-   **Multiple Clients:** The core API is designed to be consumed by different clients. The primary client is `orgzr-cli` for direct, scriptable interaction.
-   **Local-First Data:** All data is intended to be stored locally, giving you full control and privacy. (Database integration is planned).