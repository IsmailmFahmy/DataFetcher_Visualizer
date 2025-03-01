import sqlite3
import matplotlib.pyplot as plt
from datetime import datetime
import json
import sys


# Fetch data
def fetch_initial_data(ticker):
    # Connect to the database
    conn = sqlite3.connect("../stock_data.db")
    cursor = conn.cursor()

    # SQL to fetch the time & prices for all entries with a certain ticker
    query = f"SELECT unixtime, currentprice FROM stockdata WHERE ticker = '{ticker}' ORDER BY unixtime"

    cursor.execute(query)
    data = cursor.fetchall()
    conn.close()
    return data



def fetch_last_record(ticker):
    # Connect to the database
    conn = sqlite3.connect("stock_data.db")
    cursor = conn.cursor()

    # SQL to fetch the latest added entry with the same criteria
    cursor.execute("""SELECT unixtime, currentprice FROM stockdata 
                   WHERE ticker = ?
                   ORDER BY unixtime DESC 
                   LIMIT 1""",
                   (ticker,))


    data = cursor.fetchall()
    conn.close()

    return data



def plot_graph(data, ticker):

    # Convert data for plotting
    timestamps = [row[0] for row in data]
    price = [row[1] for row in data]

    # start_time = timestamps[0]
    # end_time = timestamps[-1]

    # Create plot
    plt.figure(figsize=(10, 5))


    timestamps = [datetime.fromtimestamp(row[0]).strftime("%d-%m\n%H:%M") for row in data]


    #       y values    x values
    plot = plt.plot(timestamps, price, marker=".", linestyle="-", color="b", label="Data Trend")

    plt.xticks([                                            # Editing the x axis 
        f"{date}" if (i%2==0) or (date == timestamps[-1])   # Only display the date for every 2 entries (every 30 min)
        else " "                                            # Otherwise, show an empty space
        for (i, date) in enumerate(timestamps)])


    plt.xlabel("Time")
    plt.ylabel("Price")

    # plt.title(f"Price of {ticker} from {start_time} to {end_time}")

    plt.legend()
    plt.grid()

    # Show the plot
    plt.show()
    return timestamps, price, plot

def update_graph(timestamps,price, new_data, plot):
    new_time = datetime.fromtimestamp(new_data[0][0]).strftime("%d-%m\n%H:%M")
    timestamps.append(new_time)
    price.append(new_data[1])
    plot.set_ydata(timestamps)
    plot.set_xdata(price)
    plt.draw()

def read_stdin():
    try:
        msg = json.loads(line.strip())  # Read JSON from stdin
        ticker = msg.get("ticker", "")
        # command = msg.get("type", "")
        signal = msg.get("signal", "")
        return ticker, signal


    except Exception as e:
        print(f"Error processing input: {e}", file=sys.stderr)

    return "error", "error"




if __name__ == "__main__":


    for line in sys.stdin:
        print("reading again")
        ticker, signal = read_stdin()
        print(f"ticker -> {ticker}, signal -> {signal}")
        if signal == "start":
            initial_data = fetch_initial_data(ticker)
            time, price, plot = plot_graph(initial_data, ticker)

        elif signal == "update":
            print("fetching last record")
            new_data = fetch_last_record(ticker)

            print("updating graph")
            update_graph(time, price, new_data, plot)



if __name__ == "__main__":
    plot_data = {}

