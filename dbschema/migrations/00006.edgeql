CREATE MIGRATION m1jlxdp3bbeex3koxmdrlsyp5lwycrahkpfttu6vfgximsdfx22bwa
    ONTO m1owry3ztz6ypwxgx2u5qp5fvhckxaypsabzy6e3qdx5gvo7nzx7sa
{
  ALTER TYPE Discord::User {
      DROP CONSTRAINT std::exclusive ON ((.username ++ .discriminator));
  };
  ALTER TYPE Discord::User {
      CREATE CONSTRAINT std::exclusive ON ((.username, .discriminator));
  };
};
