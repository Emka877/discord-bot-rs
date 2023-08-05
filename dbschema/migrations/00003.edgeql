CREATE MIGRATION m1tyltixelxbe2yhtwyibdhmsfhcsgvbqizt7fxsdjt3ll7t7wsypq
    ONTO m1kcmf7uwqttbs6p2roomlmc5gkxeyap5mj32znrtx26lamzy332ca
{
  ALTER TYPE Discord::User {
      ALTER PROPERTY discriminator {
          CREATE CONSTRAINT std::exclusive;
      };
      ALTER PROPERTY uniqueId {
          CREATE CONSTRAINT std::exclusive;
      };
      ALTER PROPERTY username {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
