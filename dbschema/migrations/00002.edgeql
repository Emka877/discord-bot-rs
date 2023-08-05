CREATE MIGRATION m1kcmf7uwqttbs6p2roomlmc5gkxeyap5mj32znrtx26lamzy332ca
    ONTO m1sdmteesiraezs4f3uidxklm6pdoqz2mx7fu4zul4m2y2sd3nskeq
{
  ALTER TYPE Discord::User {
      ALTER PROPERTY discriminator {
          SET readonly := true;
      };
      ALTER PROPERTY uniqueId {
          SET readonly := true;
      };
      ALTER PROPERTY username {
          SET readonly := true;
      };
  };
};
