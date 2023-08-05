CREATE MIGRATION m1nuxh6uukyijrpjkmeuwrfc4zls63po7n3udxf2n4jg3th3zjj2ba
    ONTO m1rpd3jv36b62gpx4dp4b33fadidt3qf6yersimupqnq6iblkkfh2a
{
  ALTER TYPE Discord::User {
      DROP CONSTRAINT std::exclusive ON ((.username ++ .discriminator));
  };
  ALTER TYPE Discord::User {
      DROP PROPERTY discriminator;
  };
  ALTER TYPE Discord::User {
      ALTER PROPERTY uniqueId {
          RENAME TO unique_id;
      };
      ALTER PROPERTY username {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
