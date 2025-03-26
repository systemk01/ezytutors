/* Delete the course with course_id 3 for multiple testing, because
when the course with id=3 ist created once, the second test leads to
an error, because the course_id already exists*/
DELETE FROM ezy_course_c4 WHERE course_id = 3;