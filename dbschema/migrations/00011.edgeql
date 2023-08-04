CREATE MIGRATION m1xzl7flqtqtslykb5vc3ztuo2mcjvl3ljz2qs23uq3aiyoyefw7jq
    ONTO m1nuxh6uukyijrpjkmeuwrfc4zls63po7n3udxf2n4jg3th3zjj2ba
{
  ALTER TYPE Discord::User {
      CREATE REQUIRED PROPERTY display_name: std::str {
          SET REQUIRED USING (<std::str>{'unspecified'});
      };
  };
};
