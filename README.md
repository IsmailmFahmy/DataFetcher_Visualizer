This project visualizes the stock price trend of a certain ticker.

# How it works
## Data collection
Here, rust is used to send a get request to an stock price api to returieve the data.
The data is recieved as json, which is then deserialized into a rust struct.

## Data storage
Data is stored in an SQLite database stored in the repository.
Diesel (an ORM) is used to manage the database and insert data.

## Visualization
Matplotlib in Python is used to visualize the data stored in the database.

## Scheduling and Communication
Every 15 minutes, the program sends an API get Request and stores the data.
After which a json message sent over a pipe is sent to the python script.
This json message tells the python script to either:
1. Initialize the graph using data in the database.
2. Fetch the last entry and update the graph.
It also includes the ticker of the stock so correct data can be queried.

![image|300](https://github.com/user-attachments/assets/1f1c5950-7d72-4e83-818d-3950041e99e5)
