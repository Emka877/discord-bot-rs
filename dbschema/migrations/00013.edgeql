CREATE MIGRATION m1nozqt4iowmhp6i3vgc6onyy2ia2dapz7ws6pc7lsbkxbsjff7mja
    ONTO m1i6zbmyhligvqobjcy3ajwjapon3wniye2kkthuuowzd24tur4zkq
{
  ALTER TYPE Discord::ChannelMessage {
      ALTER LINK author {
          RESET OPTIONALITY;
      };
  };
};
