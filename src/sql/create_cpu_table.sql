-- USE oscar_utilization;

CREATE TABLE cpu (
    id          integer primary key autoincrement,
    time        timestamp,
    allocated   int, 
    idle        int,
    other       int, 
    total       int
);