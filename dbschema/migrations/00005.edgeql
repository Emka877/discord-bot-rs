CREATE MIGRATION m1owry3ztz6ypwxgx2u5qp5fvhckxaypsabzy6e3qdx5gvo7nzx7sa
    ONTO m14qusoale5jooiv7l2ldn62f7twg6vz5crhzmrclkeq5cvxr5kwla
{
  ALTER TYPE Discord::User {
      CREATE CONSTRAINT std::exclusive ON ((.username ++ .discriminator));
  };
  ALTER TYPE Discord::User {
      DROP CONSTRAINT std::exclusive ON ((.username, .discriminator));
  };
};
