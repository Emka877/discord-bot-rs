CREATE MIGRATION m1qoxgtfeaqbydu5fi2vfbfgeibl4ya5u2tpjy6462no3jt65fxpya
    ONTO m1fgo7ey37s343utmi6ccw6toopkaklmph6rzlavq3l5x3dvl2uvia
{
  CREATE TYPE Discord::PortfolioLine {
      CREATE PROPERTY bought_at: std::float64;
      CREATE REQUIRED PROPERTY created_at: cal::local_datetime {
          SET default := (cal::to_local_datetime(std::datetime_current(), 'Europe/Brussels'));
      };
      CREATE REQUIRED PROPERTY quantity: std::float64;
      CREATE REQUIRED PROPERTY ticker: std::str {
          SET readonly := true;
      };
  };
  CREATE TYPE Discord::Portfolio {
      CREATE MULTI LINK lines: Discord::PortfolioLine;
  };
  ALTER TYPE Discord::User {
      CREATE SINGLE LINK portfolio: Discord::Portfolio {
          SET default := (<Discord::Portfolio>{});
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
