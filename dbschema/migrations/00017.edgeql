CREATE MIGRATION m14phrzo3cotlvdic5jjygv2d466kvpygp3gvz2ivgpzvsqpn3znga
    ONTO m1qoxgtfeaqbydu5fi2vfbfgeibl4ya5u2tpjy6462no3jt65fxpya
{
  ALTER TYPE Discord::Portfolio {
      DROP LINK lines;
  };
  ALTER TYPE Discord::User {
      DROP LINK portfolio;
  };
  DROP TYPE Discord::Portfolio;
  DROP TYPE Discord::PortfolioLine;
};
