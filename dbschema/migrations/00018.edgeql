CREATE MIGRATION m1e6xgwdi472lpxhekjegelzs5v3wrnqpxana4dtzl5ltwgvdls3sq
    ONTO m14phrzo3cotlvdic5jjygv2d466kvpygp3gvz2ivgpzvsqpn3znga
{
  CREATE TYPE Discord::PortfolioLine {
      CREATE REQUIRED PROPERTY bought_at: std::float64;
      CREATE REQUIRED PROPERTY created_at: cal::local_datetime {
          SET default := (cal::to_local_datetime(std::datetime_of_statement(), 'Europe/Brussels'));
      };
      CREATE REQUIRED PROPERTY quantity: std::float64;
      CREATE REQUIRED PROPERTY ticker: std::str;
  };
};
