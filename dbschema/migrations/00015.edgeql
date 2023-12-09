CREATE MIGRATION m1fgo7ey37s343utmi6ccw6toopkaklmph6rzlavq3l5x3dvl2uvia
    ONTO m1bsofimbc3xjzkzwzli2hosl5lamtfvrkrltynbsyd53mkp7yw4oq
{
  ALTER TYPE Discord::User {
      CREATE REQUIRED PROPERTY money: std::float64 {
          SET default := 1000.0;
      };
  };
};
