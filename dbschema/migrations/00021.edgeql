CREATE MIGRATION m1r5p5y26v3iaivl2qngprjowfu7breg63yopgvjkw7d4iphz4hafa
    ONTO m1g6uucpsu47ceqkjvcohiyai5eyyw65svdi6getesqqwbxpmz5lwq
{
  ALTER TYPE Discord::User {
      CREATE SINGLE LINK portfolio: Discord::Portfolio {
          SET default := (<Discord::Portfolio>{});
      };
  };
  ALTER TYPE Discord::Portfolio {
      CREATE MULTI LINK owner := (.<portfolio[IS Discord::User]);
  };
};
