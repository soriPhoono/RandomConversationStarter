use rand::{seq::IteratorRandom, thread_rng};
use roux::Subreddit;

pub async fn get_random_post_url(subreddit_name: &str) -> Option<String> {
    match Subreddit::new(subreddit_name).latest(10, None).await {
        Ok(latest_posts) => match latest_posts.data.children.iter().choose(&mut thread_rng()) {
            Some(post) => post.data.url.clone(),
            None => {
                println!("Failed to get a random post as no posts were given to select from");

                None
            }
        },
        Err(why) => {
            println!(
                "Failed to find top 10 posts from given sub: {}\n{}",
                subreddit_name, why
            );

            None
        }
    }
}
