use chrono::{DateTime, Duration, NaiveTime, TimeDelta, TimeZone, Utc};
use serde::Deserialize;
use serenity::all::{ExecuteWebhook, Http, Webhook};

#[derive(Deserialize, Debug)]
struct Config {
    channel_id: String,
    webhook_id: String,
}

enum EventType {
    CoreTyria,
    LWS1,
    LWS2,
    HoT,
    LWS3,
    PoF,
    LWS4,
    IcebroodSaga,
    EoD,
    SotO,
}

struct GW2Event {
    start_time: Option<NaiveTime>,
    interval: Duration,
    event_type: EventType,
    chat_link: String,
    next_occurrence: Option<NaiveTime>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = create_config().await?;

    let events = register_events()?;

    for event in events {
        for i in 0..16 {
            let time = event.start_time.unwrap() + event.interval;
            println!("{}", time);
        }
    }

    let http = Http::new("");

    let url = format!(
        "https://discord.com/api/webhooks/{}/{}",
        config.channel_id, config.webhook_id
    );

    let webhook = Webhook::from_url(&http, &url).await?;

    let builder = ExecuteWebhook::new().content("hi").username("GW2 Alerts");

    //webhook.execute(&http, false, builder).await?;

    Ok(())
}

fn register_events() -> Result<Vec<GW2Event>, Box<dyn std::error::Error>> {
    let mut events: Vec<GW2Event> = vec![];

    // Triple Trouble
    events.push(GW2Event {
        start_time: NaiveTime::from_hms_opt(1, 0, 0),
        interval: Duration::hours(4),
        event_type: EventType::CoreTyria,
        chat_link: "".to_owned(),
    });

    Ok(events)
}

async fn create_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = tokio::fs::read("config.toml").await?;

    let data: String = String::from_utf8_lossy(&file).parse()?;

    Ok(toml::from_str(&data)?)
}
