use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // println!("message: {}",fetch_video_frames().await);
    let (video_message,audio_message) = tokio::join!(fetch_video_frames(), fetch_audio_track());

    println!("video: {} | audio: {}",video_message,audio_message);
    
    println!(" END ");

}



async fn fetch_video_frames()->String {
    println!("fetching frames");
    sleep(Duration::from_secs(5)).await;
    String::from("120 frames fetched")

}

async fn fetch_audio_track() -> String {
    println!("fetching audio");
    sleep(Duration::from_secs(4)).await;
    String::from("audio track")
}