alter table users add column confirmed boolean default false;

create table email_confirmation_tokens (
    token text not null constraint token_pk primary key,
    user integer not null constraint user_fk references users (id)
);

create unique index token_ui on email_confirmation_tokens (token);

create unique index user_ui on email_confirmation_tokens (user);