pub fn setup_vars() {
    std::env::set_var("DATABASE_URL", "postgresql://postgres:postgres@0.0.0.0:5432/api_test");
    std::env::set_var("TIDE_SECRET", "somesuperlongandhardtoreadsecretylongerthanthirtytwobytes");
    std::env::set_var("SESSION_SECRET", "DoqCX86xnQPSQX4ZZzG5V+bSYhqzo02KzM6KVXUiglk=");
    std::env::set_var("PASSWORD_HASH", "$argon2i$v=19$m=65536,t=1,p=1$c29tZXNhbHQAAAAAAAAAAA$+r0d29hqEB0yasKr55ZgICsQGSkl0v0kgwhd+U3wyRo");
}
