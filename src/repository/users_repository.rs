use crate::model::persistent::schema::users;
use crate::model::persistent::users::User;
use crate::model::persistent::pagination::Paginated;
use diesel::prelude::*;

async fn get_all_users(page: i64, per_page: i64) {
    let mut query = users::table
        .order(users::created_time_dt.desc())
        .filter(users::created_time_dt.is_not_null())
        .inner_join(users::table)
        .select((users::all_columns, (users::id, users::phone_number)))
        .paginate(1);

    if let Some(per_page) = per_page {
        use std::cmp::min;
        query = query.per_page(min(per_page, 25));
    }

    let (posts_with_user, total_pages) =
        query.load_and_count_pages::<(Post, User)>(&conn)?;
    let (posts, post_users): (Vec<_>, Vec<_>) = posts_with_user.into_iter().unzip();

    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(&conn)?
        .grouped_by(&posts);

    let to_display = posts.into_iter().zip(post_users).zip(comments);

    let id = diesel::insert_into(posts::table)
        .values((
            posts::user_id.eq(user.id),
            posts::title.eq(title),
            posts::body.eq(body),
        ))
        .returning(posts::id)
        .get_result::<i32>(&conn)?;

    use diesel::dsl::now;
    use post::Status::*;
    use schema::posts::dsl::*;

    let user = current_user(&conn)?;
    let post = Post::belonging_to(&user)
        .find(post_id)
        .first::<Post>(&conn)?;
    let new_body = editor::edit_string(&post.body)?;

    let updated_status = match post.status {
        Draft if publish => Some(published_at.eq(now.nullable())),
        _ => None,
    };

    diesel::update(&post)
        .set((body.eq(new_body), updated_status))
        .execute(&conn)?;

    use schema::comments::dsl::*;

    let inserted = diesel::insert_into(comments)
        .values((
            user_id.eq(current_user(&conn)?.id),
            post_id.eq(given_post_id),
            body.eq(editor::edit_string("")?),
        ))
        .returning(id)
        .get_result::<i32>(&conn)?;

    use schema::comments::dsl::*;

    let user = current_user(&conn)?;

    let comment = Comment::belonging_to(&user)
        .find(comment_id)
        .first::<Comment>(&conn)?;

    diesel::update(comments)
        .set(body.eq(editor::edit_string(&comment.body)?))
        .execute(&conn)?;

    let mut query = Comment::belonging_to(&user)
        .order(comments::created_at.desc())
        .inner_join(posts::table)
        .select((comments::all_columns, posts::title))
        .paginate(page);

    if let Some(per_page) = per_page {
        use std::cmp::min;
        query = query.per_page(min(per_page, 25));
    }

    let (comments_and_post_title, total_pages) =
        query.load_and_count_pages::<(Comment, String)>(&conn)?;
}