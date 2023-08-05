module Dev {
    type ErrorLog {
        required log: str;
        required created: datetime { default := datetime_current() };
        required created_local: cal::local_datetime { default := cal::to_local_datetime(datetime_current(), 'Europe/Brussels') }
        channel_name: str;
        level: str {
            constraint one_of('debug', 'error', 'warn', 'info', 'other', 'unknown')
        };
    }
}
