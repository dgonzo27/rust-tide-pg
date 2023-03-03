pub fn setup_vars() {
    std::env::set_var("DATABASE_URL", "postgresql://postgres:postgres@0.0.0.0:5432/api_test");
    std::env::set_var("TIDE_SECRET", "somesuperlongandhardtoreadsecretylongerthanthirtytwobytes");
}
