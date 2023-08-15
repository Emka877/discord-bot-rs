module Discord {
    type User {
        required username: str {
            readonly := true;
            constraint exclusive;
        }

        required unique_id: str {
            readonly := true;
            constraint exclusive;
        }

        required display_name: str;
    }

    type ChannelMessage {
        channel_id: str {
            readonly := true;
        }

        required is_bot: bool {
            readonly := true;
            default := false;
        }

        required message: str {
            readonly := true;
        }

        required created_local: cal::local_datetime { 
            default := cal::to_local_datetime(datetime_current(), 'Europe/Brussels')
        }

        author: Discord::User {
            readonly := true;
        }
    }
}
