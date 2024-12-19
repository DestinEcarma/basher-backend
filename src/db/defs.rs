use std::sync;

pub type DB = surrealdb::Surreal<surrealdb::engine::any::Any>;
pub type SharedDB = sync::Arc<DB>;

pub struct DBTable;

impl DBTable {
    pub const TAG: &'static str = "tag";
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
        time::millis(time.created_at) AS created_at,
        ((SELECT VALUE meta::id(out) FROM ->tag_line)) AS tags,
        {
            is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
            is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
            is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
            identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $parent.id LIMIT 1)
        } AS user_status
        OMIT time
    FROM topic
    ORDER BY created_at DESC
    LIMIT 20
    START $offset
    FETCH counter;
    "#;

    pub const SELECT_ONLY_TOPIC: &'static str = r#"
    BEGIN TRANSACTION;
    
    UPDATE ($topic).counter SET views = views + 1; 

    RETURN (
        SELECT
            *,
            meta::id(id) AS id,
            time.created_at AS activity,
            (SELECT VALUE meta::id(out) FROM ->tag_line) AS tags,
            {
                is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
                is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
                is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
                identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1)
            } AS user_status
            OMIT time
        FROM ONLY $topic
        LIMIT 1
        FETCH counter
    );

    COMMIT TRANSACTION;
    "#;

    pub const SELECT_ONLY_REPLY: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        time::millis(time.created_at) AS created_at,
        (SELECT meta::id($parent.parent) AS id, identity AS user_identity FROM ONLY parent<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1) AS parent,
        {
            is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
            is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
            is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
            identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1)
        } AS user_status
        OMIT time
    FROM ONLY $reply
    LIMIT 1
    FETCH counter;
    "#;

    pub const SELECT_REPLIES_FROM_TOPIC: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        time::millis(time.created_at) AS created_at,
        (SELECT meta::id($parent.parent) AS id, identity AS user_identity FROM ONLY parent<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1) AS parent,
        {
            is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
            is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
            is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
            identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1)
        } AS user_status
        OMIT time
    FROM $topic->contains.out
    ORDER BY created_at
    LIMIT 10
    START $offset
    FETCH counter;
    "#;

    pub const SELECT_REPLIES_FROM_REPLY: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        time.created_at AS activity,
        time::millis(time.created_at) AS created_at,
        (SELECT meta::id($parent.parent) AS id, identity AS user_identity FROM ONLY parent<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1) AS parent,
        {
            is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
            is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
            is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
            identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $topic LIMIT 1)
        } AS user_status
        OMIT time
    FROM reply
    WHERE parent = $reply
    ORDER BY created_at
    LIMIT 10
    START $offset
    FETCH counter
    "#;

    pub const SELECT_TOPICS_FROM_QUERY: &'static str = r#"
    SELECT
        *,
        meta::id(id) AS id,
        search::score(1) AS score,
        time.created_at AS activity,
        time::millis(time.created_at) AS created_at,
        ((SELECT VALUE meta::id(out) FROM ->tag_line)) AS tags,
        {
            is_owner: ((SELECT * FROM ONLY id<-wrote WHERE in = $user LIMIT 1) != NONE),
            is_shared: ((SELECT * FROM ONLY id<-shares WHERE in = $user LIMIT 1) != NONE),
            is_liked: ((SELECT * FROM ONLY id<-likes WHERE in = $user AND is_deleted = false LIMIT 1) != NONE),
            identity: (SELECT VALUE identity FROM ONLY id<-wrote<-user<-user_identity WHERE in = $parent.id LIMIT 1)
        } AS user_status
        OMIT time
    FROM (SELECT *, search::score(1) AS score FROM topic WHERE $query = "" OR title @1@ $query)
    WHERE array::is_empty($tags) OR id->tag_line[WHERE meta::id(out) IN $tags]
    ORDER BY score
    LIMIT 20
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
    BEGIN TRANSACTION;

    LET $topic = (CREATE ONLY topic CONTENT {
    	content: $content,
    	title: $title
    });
    
    RELATE $user -> wrote -> $topic;
    RELATE $topic -> tag_line -> (INSERT INTO tag $tags ON DUPLICATE KEY UPDATE indexed += 1);
    
    RETURN meta::id($topic.id);

    COMMIT TRANSACTION;
    "#;

    pub const CREATE_REPLY: &'static str = r#"
    BEGIN TRANSACTION;
    
    LET $reply = (CREATE ONLY reply CONTENT {
    	content: $content,
    	parent: $parent
    });
    
    RELATE $user -> wrote -> $reply;
    RELATE $topic -> contains -> $reply;
    
    IF ((SELECT * FROM ONLY $topic->user_identity WHERE out = $user LIMIT 1) = NONE) {
    	(RELATE $topic -> user_identity -> $user SET identity = $topic.counter.users);
    };
    
    RETURN meta::id($reply.id);
    
    COMMIT TRANSACTION;
    "#;

    pub const UPDATE_TOPIC: &'static str = r#"
    BEGIN TRANSACTION;
   
    IF ((SELECT * FROM ONLY $topic<-wrote WHERE in = $user LIMIT 1) = NONE) {
        RETURN NONE;
    };
    
    UPDATE ONLY $topic SET content = $content, title = $title;
     
    RELATE $topic -> tag_line -> (
        SELECT *
        FROM (INSERT INTO tag $tags ON DUPLICATE KEY UPDATE indexed += 1)
        WHERE (SELECT VALUE id FROM ONLY $topic->tag_line WHERE out = $parent.id LIMIT 1) = NONE
    );
    
    RETURN meta::id($topic.id);
    
    COMMIT TRANSACTION;
    "#;

    pub const UPDATE_REPLY: &'static str = r#"
    BEGIN TRANSACTION;

    IF ((SELECT * FROM ONLY $reply<-contains WHERE in = $topic LIMIT 1) = NONE) {
        RETURN NONE;
    };

    IF ((SELECT * FROM ONLY $reply<-wrote WHERE in = $user LIMIT 1) = NONE) {
    	RETURN NONE;
    };

    UPDATE ONLY $reply SET content = $content;

    RETURN meta::id($reply.id);

    COMMIT TRANSACTION;
    "#;

    pub const LIKE_POST: &'static str = r#"
    BEGIN TRANSACTION;
    
    IF ((SELECT * FROM ONLY $post LIMIT 1) = NONE) {
        RETURN NONE;
    };
    
    INSERT RELATION INTO likes {
        in: $user,
        out: $post
    } ON DUPLICATE KEY UPDATE is_deleted = !is_deleted;
    
    RETURN meta::id($post.id);
    
    COMMIT TRANSACTION;
    "#;

    pub const SHARE_POST: &'static str = r#"
    BEGIN TRANSACTION;
    
    IF ((SELECT * FROM ONLY $post LIMIT 1) = NONE) {
        RETURN NONE;
    };
    
    IF ((SELECT * FROM ONLY $user->shares WHERE out = $post LIMIT 1) != NONE) {
        RETURN meta::id($post.id);
    };
    
    INSERT RELATION INTO shares {
        in: $user,
        out: $post
    };
    
    RETURN meta::id($post.id);
    
    COMMIT TRANSACTION;
    "#;
}
