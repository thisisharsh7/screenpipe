CREATE VIRTUAL TABLE frames_fts USING fts5(name, app_name);
INSERT INTO frames_fts VALUES ('frame1', 'zoom.us');
SELECT * FROM frames_fts WHERE frames_fts MATCH 'app_name:"zoom.us"';
