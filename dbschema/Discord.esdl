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
}
