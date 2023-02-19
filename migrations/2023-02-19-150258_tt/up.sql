-- Your SQL goes here
CREATE TABLE posts (
                        id1 INTEGER NOT NULL,
                        id2 INTEGER NOT NULL,
                        title VARCHAR NOT NULL,
                        body TEXT NOT NULL,
                        published INTEGER NOT NULL DEFAULT 0,
                        PRIMARY KEY (id1, id2)
);
create index tt on posts (id1,published)