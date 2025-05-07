use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client_derive_encode::EncodeLabelSet;
use std::sync::atomic::AtomicU32;
use std::sync::OnceLock;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    pub value_type: String,
}

#[derive(Debug)]
pub struct Metrics {
    pub(crate) temperature: Family<Labels, Gauge<f32, AtomicU32>>,
    pub(crate) fan_speed: Family<Labels, Gauge<f32, AtomicU32>>,
}

impl Metrics {
    pub fn set_temperature(&self, temperature: f32) {
        self.temperature
            .get_or_create(&Labels {
                value_type: String::from("temperature"),
            })
            .set(temperature);
    }

    pub fn set_fan_speed(&self, fan_speed: f32) {
        self.fan_speed
            .get_or_create(&Labels {
                value_type: String::from("fan_speed"),
            })
            .set(fan_speed);
    }
}

pub fn get_fan_metrics() -> &'static Metrics {
    static FAN_METRICS: OnceLock<Metrics> = OnceLock::new();
    FAN_METRICS.get_or_init(|| Metrics {
        temperature: Family::default(),
        fan_speed: Family::default(),
    })
}
