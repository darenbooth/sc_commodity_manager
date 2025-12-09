# 🚀 Star Citizen Commodity Tracker (CLI)
## Project Overview

This is a persistent, command-line application written in Rust designed specifically for Star Citizen traders. It helps you keep track of your goods, manage your expenses, and calculate your trading profits.

Your inventory data is automatically saved to a local file (inventory_data.json) between sessions, so you don't lose track of your assets.
## Key Features ✨

* Multi-Asset Tracking: Keep track of the inventory, total cost, and average cost for any number of different commodities (e.g., Astatine, Laranite, Gold).

* Simple Transactions: Easily record Buy (B) and Sell (S) operations.

* Net Profit/Loss: Calculate your final profit on sales by subtracting the original cost and associated Transportation Fees from the revenue.

* Automatic Cost Basis: The program automatically updates the cost of your remaining inventory using the Average Cost Method.

* Session Summary: Displays the total Net Profit/Loss accumulated during the time you've had the program open.

* Data Persistence: Your inventory data is saved and loaded automatically using a local JSON file.

## Installation & Setup

Download the Executable:

1. Go to the Releases page for this repository on GitHub.

2. Look for the Assets section under the latest version.

3. Download the file that matches your operating system (e.g., the file ending in .exe for Windows).

4. Unzip and Place: Unzip the downloaded file (windows) and place the main executable (named star_citizen_commodity_tracker or star_citizen_commodity_tracker.exe) in a simple, easy-to-access folder.

## Running the Tracker

### Windows
1. Simply just open up the .exe file with mouse click/s

### Linux/macOS:
1. Open the terminal.
2. Use the cd command to navigate to the executable's folder.
3. Type: `chmod +x` to make the file executable.
4. Type: `./sc_commodity_tracker`.

Once started, the tracker will welcome you and ask for your first transaction.

## How to Use 🧭

The program runs in a continuous loop, asking you what you want to do next.

Start a Transaction: Type B to buy commodities or S to sell them.

Save and Exit: Type Q to quit the application. This is essential because the program only saves your data when you quit.

## How the Costs are Handled

When you Buy, the total cost you pay plus any transportation fees are added to the cost basis of your inventory.

When you Sell, the program calculates the Net Profit/Loss for that sale:
Net Profit/Loss=Sale Revenue−Cost of Goods Sold (COGS)−Transport Fees.

## Contributing

If you want to contribute to the code, fix bugs, or suggest new features (like integrating specific Star Citizen price data), feel free to submit a pull request!

## License
This project is licensed under the MIT License.
