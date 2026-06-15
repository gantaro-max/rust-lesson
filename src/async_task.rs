use tokio::{join, time};

async fn boil_water() -> &'static str {
    time::sleep(time::Duration::from_secs(2)).await;
    "お湯が沸いた"
}

async fn chop_vegetables() -> &'static str {
    time::sleep(time::Duration::from_secs(1)).await;
    "野菜が切れた"
}

async fn prepare_sauce() -> &'static str {
    time::sleep(time::Duration::from_secs(3)).await;
    "ソースができた"
}

pub async fn run() {
    let start = std::time::Instant::now();
    let (boiled, chopped, prepared) = join!(boil_water(), chop_vegetables(), prepare_sauce());
    println!("{}", boiled);
    println!("{}", chopped);
    println!("{}", prepared);

    println!("経過時間: {:?}", start.elapsed());
}
