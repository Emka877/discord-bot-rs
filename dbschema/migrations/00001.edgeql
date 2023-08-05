CREATE MIGRATION m1sdmteesiraezs4f3uidxklm6pdoqz2mx7fu4zul4m2y2sd3nskeq
    ONTO initial
{
  CREATE MODULE Discord IF NOT EXISTS;
  CREATE TYPE Discord::User {
      CREATE REQUIRED PROPERTY discriminator: std::str;
      CREATE REQUIRED PROPERTY uniqueId: std::str;
      CREATE REQUIRED PROPERTY username: std::str;
  };
};
