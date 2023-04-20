START TRANSACTION;

DROP TABLE IF EXISTS global_comments;

CREATE TABLE global_comments (
  id SERIAL PRIMARY KEY,
  body TEXT,
  author TEXT
);

INSERT INTO global_comments (body, author)
VALUES (
  'Great fleas have little fleas upon their backs to bite ''em,
And little fleas have lesser fleas, and so ad infinitum.
And the great fleas themselves, in turn, have greater fleas to go on;
While these again have greater still, and greater still, and so on.',
  'Morgan'
);

COMMIT;