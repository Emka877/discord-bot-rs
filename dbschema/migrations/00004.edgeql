CREATE MIGRATION m14qusoale5jooiv7l2ldn62f7twg6vz5crhzmrclkeq5cvxr5kwla
    ONTO m1tyltixelxbe2yhtwyibdhmsfhcsgvbqizt7fxsdjt3ll7t7wsypq
{
  ALTER TYPE Discord::User {
      CREATE CONSTRAINT std::exclusive ON ((.username, .discriminator));
      ALTER PROPERTY discriminator {
          DROP CONSTRAINT std::exclusive;
      };
      ALTER PROPERTY username {
          DROP CONSTRAINT std::exclusive;
      };
  };
};
