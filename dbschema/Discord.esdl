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

        required money: float64 {
            default := 1000.0
        }

        single link portfolio: Portfolio {
            default := <Discord::Portfolio>{}
        }
    }

    type Portfolio {
        multi link owner := (.<portfolio[is User]);
        multi lines: PortfolioLine;
    }

    type PortfolioLine {
        multi link belongs_to := (.<lines[is Portfolio]);

        required ticker: str;
        required quantity: float64;
        required bought_at: float64;
        required created_at: cal::local_datetime {
            default := cal::to_local_datetime(datetime_of_statement(), 'Europe/Brussels')
        }
    }
}
