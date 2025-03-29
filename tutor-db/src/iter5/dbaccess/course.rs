use crate::errors::EzyTutorError;
use crate::models::course::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    //prepare sql statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1", tutor_id
    )
    .fetch_all(pool)
    .await?;
    //extract results
    let courses: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
        .collect();
    match courses.len() {
        0 => Err(EzyTutorError::NotFound(
            "Courses not found for tutor".into(),
        )),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    //prepare sql statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1 AND course_id = $2", tutor_id, course_id
    )
    .fetch_one(pool)
    .await;
    if let Ok(course_row) = course_row {
        // Execute query
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: Course,
) -> Result<Course, EzyTutorError> {
    //posted time is not set, because in database.sql it is set to posted_time TIMESTAMP default now()
    let course_row = sqlx::query!(
        "INSERT INTO ezy_course_c5 
        (tutor_id, course_id, course_name) VALUES($1,$2,$3) 
        returning tutor_id, course_id, course_name, posted_time",
        new_course.tutor_id,
        new_course.course_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await?;
    //returning Course
    Ok(Course {
        tutor_id: course_row.tutor_id,
        course_id: course_row.course_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(course_row.posted_time.unwrap()),
    })
}
