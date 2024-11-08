use advent_of_code::startup::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    let a = app
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

    println!("{}", a);
}
