module Discord {
    type User {
        required username: str {
            readonly := true;
        }

        required discriminator: str {
           readonly := true;
        }

        required uniqueId: str {
            readonly := true;
            constraint exclusive;
        }

        constraint exclusive on ((.username ++ .discriminator));
    }
}
