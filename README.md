🚀 Star Citizen Commodity Tracker (CLI)
Project Overview

This is a simple, persistent command-line interface (CLI) application written in Rust designed to help Star Citizen traders track their commodity inventory, calculate the average cost per unit, and determine Net Profit/Loss on sales.

Since the application uses local file persistence, your inventory data is saved between sessions.
Features ✨

    Multi-Asset Tracking: Track inventory, total cost, and average cost for any number of different commodities (e.g., Astatine, Laranite, Gold).

    Transaction Processing: Supports both Buy (B) and Sell (S) transactions.

    Cost Management: Automatically calculates the cost basis for remaining inventory using the Average Cost Method.

    Net Profit Calculation: Calculates the Net Profit or Loss for each sale by subtracting the Cost of Goods Sold (COGS) and any associated Transportation Fees.

    Session Summary: Displays the total Net Profit/Loss accumulated during the current run of the program.

    Persistence: Saves and loads inventory data to a local file (inventory_data.json) to persist between uses.

Installation & Setup

You can download a pre-compiled, optimized version of the application directly from GitHub Releases.

    Go to Releases: Navigate to the Releases page for this repository on GitHub.

    Download the Asset: Under the latest version, look for the Assets section and download the file corresponding to your operating system:

        Windows: Download the file ending in .exe.

        Linux/macOS: Download the file without an extension (or one labeled for your specific OS).

    Extract: Unzip the downloaded file (if necessary) and place the executable in a convenient location (like a dedicated StarCitizenTracker folder).

Running the Application

The application is run directly from your system's command line or terminal.
Platform	Command to Run
Windows	1. Open Command Prompt or PowerShell. 2. Navigate to your application folder using cd path\to\folder. 3. Execute: .\star_citizen_commodity_tracker.exe
Linux/macOS	1. Open Terminal. 2. Navigate to your application folder using cd path/to/folder. 3. Execute: ./star_citizen_commodity_tracker

Once executed, the application will greet you and prompt you for the first transaction.
How to Use 🧭

The program runs in a continuous loop, prompting you for transaction details.
Command	Action	Description
B	Buy	Increases inventory and adds the purchase cost (plus fees) to the total cost.
S	Sell	Decreases inventory, calculates Net Profit/Loss, and updates the remaining inventory cost basis.
Q	Quit	Exits the program and saves the current inventory state to inventory_data.json. This is necessary to save your progress!
Data Persistence

    Upon first run, the program creates an empty inventory_data.json file in the same directory as the executable.

    The application loads this file when starting.

    The application saves the updated inventory to this file when you quit (Q).

    DO NOT manually edit inventory_data.json while the program is running.

Data Structure & Calculations

The core data is stored in a HashMap where the key is the asset name (String), and the value is the inventory data.
Key Calculations:

    Average Cost Per Unit: Calculated as Quantity in InventoryTotal Cost in Inventory​

    Cost of Goods Sold (COGS): Calculated as Quantity Sold×Average Cost Per Unit

    Net Profit/Loss: Calculated as Total Sale Revenue−COGS−Transport Fees

Contributing

If you'd like to improve this project, feel free to submit a pull request! Any suggestions for improving the logic, adding features (like a batch/FIFO tracking method), or enhancing the CLI are welcome.
License

This project is licensed under the MIT License. See the LICENSE file for details.
