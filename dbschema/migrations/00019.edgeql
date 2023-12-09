CREATE MIGRATION m1buycnw434opafbnyh2c74au7q7wc5goqjjbkpna7s3werglo2fua
    ONTO m1e6xgwdi472lpxhekjegelzs5v3wrnqpxana4dtzl5ltwgvdls3sq
{
  CREATE TYPE Discord::Portfolio {
      CREATE MULTI LINK lines: Discord::PortfolioLine;
  };
};
