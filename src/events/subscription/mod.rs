use super::*;
use crate::*;

pub use self::setup_subscription::*;
pub use self::update_subscription_rate::*;
pub use self::pause_subscription::*;
pub use self::activate_subscription::*;
pub use self::subscribe::*;
pub use self::unsubscribe::*;
pub use self::update_fee::*;

mod setup_subscription;
mod update_subscription_rate;
mod pause_subscription;
mod activate_subscription;
mod subscribe;
mod unsubscribe;
mod update_fee;

/// Define the event variants for subscription events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionEventKind {
    SetupSubscription(SetupSubscriptionEvent),
    UpdateSubscriptionRate(UpdateSubscriptionRateEvent),
    PauseSubscription(PauseSubscriptionEvent),
    ActivateSubscription(ActivateSubscriptionEvent),
    Subscribe(SubscribeEvent),
    Unsubscribe(UnsubscribeEvent),
    UpdateFee(UpdateFeeEvent),
}

/// Define the main SubscriptionEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct SubscriptionEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: SubscriptionEventKind,
}

impl SubscriptionEvent {
    pub fn new(event: SubscriptionEventKind) -> Self {
        SubscriptionEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for SubscriptionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}
