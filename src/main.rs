use advent_of_code::startup::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    let _a = app
        .get_input(
            app.years
                .last()
                .clone()
                .expect("Should have at least one year"),
            app.days
                .last()
                .clone()
                .expect("Should have at least one day"),
        )
        .await;

    app.run_last().await.expect("");
}
