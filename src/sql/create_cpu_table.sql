USE oscar_utilization;

CREATE TABLE cpu (
    id          int primary key auto_increment,
    time        timestamp,
    allocated   int, 
    idle        int,
    other       int, 
    total       int
);