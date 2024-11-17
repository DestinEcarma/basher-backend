use std::sync;

pub type DB = surrealdb::Surreal<surrealdb::engine::any::Any>;

pub struct DBTable;

impl DBTable {
    pub const USER: &'static str = "user";
    pub const TOPIC: &'static str = "topic";
    pub const REPLY: &'static str = "reply";
}

pub struct DBQuery;

impl DBQuery {
    // Select Queries
    pub const SELECT_ID: &'static str = r#"
    SELECT * FROM ONLY $thing;
    "#;

    pub const SELECT_ONLY_USER_FROM_EMAIL: &'static str = r#"
    SELECT * FROM ONLY user WHERE email = $email LIMIT 1;
    "#;

    pub const SELECT_TOPICS: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        (SELECT VALUE out.name FROM ->tag_line) AS tags,
        time::millis(time.created_at) AS created_at
        OMIT time
    FROM topic
    ORDER BY created_at DESC
    LIMIT 10
    START $offset
    FETCH counter;
    "#;

    pub const SELECT_ONLY_TOPIC: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        (SELECT VALUE out.name FROM ->tag_line) AS tags
        OMIT time
    FROM ONLY $topic
    LIMIT 1
    FETCH counter;
    "#;

    pub const SELECT_REPLIES_FROM_TOPIC: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        (SELECT VALUE identity FROM ONLY (SELECT VALUE in FROM ONLY id<-wrote LIMIT 1)<-user_identity WHERE in = $topic LIMIT 1) AS user_index,
        (
            IF parent != NONE {
                {
                    id: meta::id(parent),
                    user_index: (SELECT VALUE identity FROM ONLY (SELECT VALUE in FROM ONLY parent<-wrote LIMIT 1)<-user_identity WHERE in = $topic LIMIT 1)
                }
            }
        ) AS parent,
        time::millis(time.created_at) AS created_at
        OMIT time
    FROM $topic->contains.out
    ORDER BY created_at
    LIMIT 10
    START $offset
    FETCH counter;
    "#;

    pub const SELECT_REPLIES_FROM_REPLY: &'static str = r#"
    LET $topic = (SELECT VALUE in FROM ONLY $reply<-contains LIMIT 1);

    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        (SELECT VALUE identity FROM ONLY (SELECT VALUE in FROM ONLY id<-wrote LIMIT 1)<-user_identity WHERE in = $topic LIMIT 1) AS user_index,
        (
            IF parent != NONE {
                {
                    id: meta::id(parent),
                    user_index: (SELECT VALUE identity FROM ONLY (SELECT VALUE in FROM ONLY parent<-wrote LIMIT 1)<-user_identity WHERE in = $topic LIMIT 1)
                }
            }
        ) AS parent,
        time::millis(time.created_at) AS created_at
        OMIT time
    FROM reply
    WHERE parent = $reply
    ORDER BY created_at
    LIMIT 10
    START $offset
    FETCH counter;
    "#;

    // Create Queries
    pub const CREATE_USER: &'static str = r#"
    CREATE ONLY user CONTENT {
        email: $email,
        password: $password
    };
    "#;

    pub const CREATE_TOPIC: &'static str = r#"
    LET $topic = (CREATE ONLY topic CONTENT {
        title: $title,
        content: $content
    } RETURN id).id;

    RELATE $topic -> tag_line -> (INSERT INTO tag $tags);
    RELATE $user -> wrote -> $topic;

    RETURN meta::id($topic);
    "#;

    pub const CREATE_REPLY: &'static str = r#"
    LET $reply = (CREATE ONLY reply CONTENT {
        content: $content,
        parent: $parent
    }).id;

    RELATE $user -> wrote -> $reply;
    RELATE $topic -> contains -> $reply;
    RELATE $topic -> user_identity -> $user SET identity = $topic.counter.users;

    RETURN meta::id($reply);
    "#;
}
