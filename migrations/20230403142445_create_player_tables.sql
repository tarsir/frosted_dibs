-- Add migration script here
CREATE TABLE IF NOT EXISTS player(
  id integer PRIMARY KEY,
  username text NOT NULL,
  level integer NOT NULL DEFAULT 1,
  experience_points integer DEFAULT 0
);

INSERT INTO player (username, level, experience_points) VALUES ("juicyfruit", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("sandbox", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("turtlehurtle", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("snakeslam", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("trueraccoon", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("kappakookoo", 1, 0);
INSERT INTO player (username, level, experience_points) VALUES ("lilrick", 1, 0);
