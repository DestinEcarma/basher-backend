-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- ANALYZERS
-- ------------------------------

DEFINE ANALYZER content_analyzer TOKENIZERS BLANK,CLASS,CAMEL,PUNCT FILTERS LOWERCASE;
DEFINE ANALYZER title_analyzer TOKENIZERS BLANK,CLASS,CAMEL,PUNCT FILTERS LOWERCASE;
DEFINE ANALYZER topic_analzyer TOKENIZERS BLANK,CLASS,CAMEL,PUNCT FILTERS SNOWBALL(ENGLISH);

-- ------------------------------
-- TABLE: contains
-- ------------------------------

DEFINE TABLE contains TYPE RELATION IN topic OUT reply SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON contains TYPE record<topic> PERMISSIONS FULL;
DEFINE FIELD out ON contains TYPE record<reply> PERMISSIONS FULL;

DEFINE INDEX topic_contains_index ON contains FIELDS in, out UNIQUE;

DEFINE EVENT increment_counter_replies ON contains WHEN $event = 'CREATE' THEN { UPDATE ONLY $value.in.counter SET replies += 1; };

-- ------------------------------
-- TABLE: counter
-- ------------------------------

DEFINE TABLE counter TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD likes ON counter TYPE int DEFAULT 0 ASSERT $value >= 0 PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD replies ON counter TYPE int DEFAULT 0 ASSERT $value >= 0 PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD shares ON counter TYPE int DEFAULT 0 ASSERT $value >= 0 PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD users ON counter TYPE option<int> PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD views ON counter TYPE option<int> PERMISSIONS FOR select, create, update WHERE FULL;

-- ------------------------------
-- TABLE: likes
-- ------------------------------

DEFINE TABLE likes TYPE RELATION IN user OUT topic | reply SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON likes TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD is_deleted ON likes TYPE bool DEFAULT false PERMISSIONS FULL;
DEFINE FIELD out ON likes TYPE record<topic | reply> PERMISSIONS FULL;

DEFINE INDEX user_likes_index ON likes FIELDS in, out UNIQUE;

DEFINE EVENT decrement_counter_likes ON likes WHEN $event = 'UPDATE' THEN { UPDATE ONLY $value.out.counter SET likes += (IF $value.is_deleted THEN -1 ELSE 1 END); };
DEFINE EVENT increment_counter_likes ON likes WHEN $event = 'CREATE' THEN { UPDATE ONLY $value.out.counter SET likes += 1; };

-- ------------------------------
-- TABLE: reply
-- ------------------------------

DEFINE TABLE reply TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD content ON reply TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD counter ON reply TYPE record<counter> DEFAULT (CREATE ONLY counter).id PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD parent ON reply TYPE option<record<reply>> PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD time ON reply TYPE object DEFAULT {  } PERMISSIONS FULL;
DEFINE FIELD time.created_at ON reply TYPE datetime DEFAULT time::now() VALUE $value OR time::now() PERMISSIONS FULL;
DEFINE FIELD time.updated_at ON reply TYPE datetime DEFAULT time::now() VALUE time::now() PERMISSIONS FULL;


DEFINE EVENT delete_counter ON reply WHEN $event = 'DELETE' THEN { DELETE $before.counter; };
DEFINE EVENT increment_parent_counter_replies ON reply WHEN $event = 'CREATE' THEN { IF $value.parent != NONE { UPDATE $value.parent.counter SET replies += 1; }; };

-- ------------------------------
-- TABLE: shares
-- ------------------------------

DEFINE TABLE shares TYPE RELATION IN user OUT topic | reply SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON shares TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON shares TYPE record<topic | reply> PERMISSIONS FULL;

DEFINE INDEX user_shares_index ON shares FIELDS in, out UNIQUE;

DEFINE EVENT increment_counter_shares ON shares WHEN $event = 'CREATE' THEN { UPDATE ONLY $value.out.counter SET shares += 1; };

-- ------------------------------
-- TABLE: tag
-- ------------------------------

DEFINE TABLE tag TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD indexed ON tag TYPE number DEFAULT 1 PERMISSIONS FULL;

-- ------------------------------
-- TABLE: tag_line
-- ------------------------------

DEFINE TABLE tag_line TYPE RELATION IN topic OUT tag SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON tag_line TYPE record<topic> PERMISSIONS FULL;
DEFINE FIELD out ON tag_line TYPE record<tag> PERMISSIONS FULL;

DEFINE INDEX topic_tag_line_index ON tag_line FIELDS in, out UNIQUE;

-- ------------------------------
-- TABLE: topic
-- ------------------------------

DEFINE TABLE topic TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD content ON topic TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD counter ON topic TYPE record<counter> DEFAULT (CREATE ONLY counter SET views = 0, users = 0).id PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD time ON topic TYPE object DEFAULT {  } PERMISSIONS FULL;
DEFINE FIELD time.created_at ON topic TYPE datetime DEFAULT time::now() VALUE $value OR time::now() PERMISSIONS FULL;
DEFINE FIELD time.updated_at ON topic TYPE datetime DEFAULT time::now() VALUE time::now() PERMISSIONS FULL;
DEFINE FIELD title ON topic TYPE string PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX topic_title_content ON topic FIELDS title, content SEARCH ANALYZER topic_analzyer BM25(1.2,0.75) DOC_IDS_ORDER 100 DOC_LENGTHS_ORDER 100 POSTINGS_ORDER 100 TERMS_ORDER 100 DOC_IDS_CACHE 100 DOC_LENGTHS_CACHE 100 POSTINGS_CACHE 100 TERMS_CACHE 100;

DEFINE EVENT delete_counter ON topic WHEN $event = 'DELETE' THEN { DELETE $before.counter; };
DEFINE EVENT delete_replies ON topic WHEN $event = 'delete' THEN { DELETE (SELECT VALUE out FROM ($value.id)->contains); };

-- ------------------------------
-- TABLE: user
-- ------------------------------

DEFINE TABLE user TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD password ON user TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD time ON user TYPE object DEFAULT {  } PERMISSIONS FULL;
DEFINE FIELD time.created_at ON user TYPE datetime DEFAULT time::now() VALUE $value OR time::now() PERMISSIONS FULL;
DEFINE FIELD time.updated_at ON user TYPE datetime DEFAULT time::now() VALUE time::now() PERMISSIONS FULL;

DEFINE INDEX user_email_index ON user FIELDS email UNIQUE;

-- ------------------------------
-- TABLE: user_identity
-- ------------------------------

DEFINE TABLE user_identity TYPE RELATION IN topic OUT user SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD identity ON user_identity TYPE option<int> DEFAULT $after.in.counter.users PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD in ON user_identity TYPE record<topic> PERMISSIONS FULL;
DEFINE FIELD out ON user_identity TYPE record<user> PERMISSIONS FULL;

DEFINE INDEX user_identity_index ON user_identity FIELDS in, out UNIQUE;
DEFINE INDEX user_identity_value_index ON user_identity FIELDS in, identity UNIQUE;

DEFINE EVENT increment_counter_users ON user_identity WHEN $event = 'CREATE' THEN { UPDATE ONLY $value.in.counter SET users += 1; };

-- ------------------------------
-- TABLE: wrote
-- ------------------------------

DEFINE TABLE wrote TYPE RELATION IN user OUT topic | reply SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON wrote TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON wrote TYPE record<topic | reply> PERMISSIONS FULL;

DEFINE INDEX user_wrote_index ON wrote FIELDS in, out UNIQUE;

DEFINE EVENT create_user_identity ON wrote WHEN $event = 'CREATE' THEN { IF meta::tb($value.out) = 'topic' { RELATE ($value.out) -> user_identity -> ($value.in) SET identity = $value.out.counter.users; }; };
DEFINE EVENT increment_counter_replies ON wrote WHEN $event = 'CREATE' THEN { IF meta::tb($value.out) = 'reply' { IF $value.out.parent != NONE { UPDATE ONLY $value.out.parent SET replies += 1; }; }; };
