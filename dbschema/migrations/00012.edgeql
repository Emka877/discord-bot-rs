CREATE MIGRATION m1i6zbmyhligvqobjcy3ajwjapon3wniye2kkthuuowzd24tur4zkq
    ONTO m1xzl7flqtqtslykb5vc3ztuo2mcjvl3ljz2qs23uq3aiyoyefw7jq
{
  CREATE TYPE Discord::ChannelMessage {
      CREATE REQUIRED LINK author: Discord::User {
          SET readonly := true;
      };
      CREATE PROPERTY channel_id: std::str {
          SET readonly := true;
      };
      CREATE REQUIRED PROPERTY created_local: cal::local_datetime {
          SET default := (cal::to_local_datetime(std::datetime_current(), 'Europe/Brussels'));
      };
      CREATE REQUIRED PROPERTY is_bot: std::bool {
          SET default := false;
          SET readonly := true;
      };
      CREATE REQUIRED PROPERTY message: std::str {
          SET readonly := true;
      };
  };
};
