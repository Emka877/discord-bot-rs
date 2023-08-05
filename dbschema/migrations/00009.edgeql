CREATE MIGRATION m1rpd3jv36b62gpx4dp4b33fadidt3qf6yersimupqnq6iblkkfh2a
    ONTO m1qfnhjcp4jwo3dvot5fwrovks4miov2vsi46rddbsydxwpl5oiwba
{
  ALTER TYPE Dev::ErrorLog {
      ALTER PROPERTY channelName {
          RENAME TO channel_name;
      };
  };
  ALTER TYPE Dev::ErrorLog {
      CREATE REQUIRED PROPERTY created_local: cal::local_datetime {
          SET default := (cal::to_local_datetime(std::datetime_current(), 'Europe/Brussels'));
      };
  };
};
