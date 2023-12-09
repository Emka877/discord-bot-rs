CREATE MIGRATION m1g6uucpsu47ceqkjvcohiyai5eyyw65svdi6getesqqwbxpmz5lwq
    ONTO m1buycnw434opafbnyh2c74au7q7wc5goqjjbkpna7s3werglo2fua
{
  ALTER TYPE Discord::PortfolioLine {
      CREATE MULTI LINK belongs_to := (.<lines[IS Discord::Portfolio]);
  };
};
