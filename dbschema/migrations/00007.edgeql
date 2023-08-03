CREATE MIGRATION m1utxiwekun2s4w6cevcswajnolledfhakqtxgzdfaf27kjjqhpvaa
    ONTO m1jlxdp3bbeex3koxmdrlsyp5lwycrahkpfttu6vfgximsdfx22bwa
{
  ALTER TYPE Discord::User {
      CREATE CONSTRAINT std::exclusive ON ((.username ++ .discriminator));
  };
  ALTER TYPE Discord::User {
      DROP CONSTRAINT std::exclusive ON ((.username, .discriminator));
  };
};
