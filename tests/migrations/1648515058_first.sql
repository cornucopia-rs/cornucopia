-- Write your migration SQL here
CREATE TABLE Author (
    Id SERIAL NOT NULL,
    Name VARCHAR(70) NOT NULL,
    Country VARCHAR(100) NOT NULL,
    PRIMARY KEY(Id)
);

CREATE TABLE Book (
    Id SERIAL NOT NULL,
    Title VARCHAR(50) NOT NULL,
    PRIMARY KEY(Id)
);
