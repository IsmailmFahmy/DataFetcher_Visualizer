CREATE TABLE stockdata (
    ticker VARCHAR(5) NOT NULL,
    unixtime INTEGER PRIMARY KEY,
    currentprice REAL NOT NULL,
    high REAL,
    low REAL,
    opening REAL,
    pclose REAL
)
