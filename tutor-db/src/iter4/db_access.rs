use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    //prepare sql statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1", tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    //extract results
    course_rows.iter()
    .map(|course_row| Course{
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap()))
    })
    .collect()
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    //prepare sql statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1 AND course_id = $2", tutor_id, course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    //create result from one course_row
    Course{
        tutor_id: course_rows.tutor_id,
        course_id: course_rows.course_id,
        course_name: course_rows.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_rows.posted_time.unwrap()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    //posted time is not set, because in database.sql it is set to posted_time TIMESTAMP default now()
    let course_rows = sqlx::query!(
        "INSERT INTO ezy_course_c5 
        (tutor_id, course_id, course_name) VALUES($1,$2,$3) 
        returning tutor_id, course_id, course_name, posted_time", 
        new_course.tutor_id, new_course.course_id, new_course.course_name
    ).fetch_one(pool)
    .await
    .unwrap();
    //returning Course
    Course{
        tutor_id: course_rows.tutor_id,
        course_id: course_rows.course_id,
        course_name: course_rows.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_rows.posted_time.unwrap()))
    }
}
