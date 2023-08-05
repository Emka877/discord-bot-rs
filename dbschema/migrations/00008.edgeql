CREATE MIGRATION m1qfnhjcp4jwo3dvot5fwrovks4miov2vsi46rddbsydxwpl5oiwba
    ONTO m1utxiwekun2s4w6cevcswajnolledfhakqtxgzdfaf27kjjqhpvaa
{
  CREATE MODULE Dev IF NOT EXISTS;
  CREATE TYPE Dev::ErrorLog {
      CREATE PROPERTY channelName: std::str;
      CREATE REQUIRED PROPERTY created: std::datetime {
          SET default := (std::datetime_current());
      };
      CREATE PROPERTY level: std::str {
          CREATE CONSTRAINT std::one_of('debug', 'error', 'warn', 'info', 'other', 'unknown');
      };
      CREATE REQUIRED PROPERTY log: std::str;
  };
};
